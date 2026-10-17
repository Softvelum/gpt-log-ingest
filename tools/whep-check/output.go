package main

import (
	"context"
	"encoding/binary"
	"fmt"
	"io"
	"math"
	"os"
	"os/exec"
	"path/filepath"
	"sort"
	"strings"
	"sync"
	"sync/atomic"
	"time"

	"github.com/pion/rtp"
	"github.com/pion/rtp/codecs"
)

// jbEntry is a buffered packet with its arrival time.
type jbEntry struct {
	pkt *rtp.Packet
	at  time.Time
}

// jitterBuffer reorders packets by sequence number. A missing packet is given
// up once the packet after it has waited for the buffer delay, which leaves
// time for NACK retransmissions to arrive.
type jitterBuffer struct {
	delay      time.Duration
	started    bool
	next, high int64
	pkts       map[int64]jbEntry
	late       atomic.Uint64 // arrived after their slot had been released
	lost       atomic.Uint64 // given up
}

const (
	jbMaxPackets = 4000
	jbMaxJump    = 10000
)

func (j *jitterBuffer) push(p *rtp.Packet, at time.Time, emit func(jbEntry, int64)) {
	if !j.started {
		j.started = true
		j.pkts = map[int64]jbEntry{}
		e := int64(p.SequenceNumber) + 1<<32
		j.next, j.high = e, e
	}
	ext := j.high + int64(int16(p.SequenceNumber-uint16(j.high)))
	if ext > j.high {
		j.high = ext
	}
	if ext < j.next {
		j.late.Add(1)
		return
	}
	if _, dup := j.pkts[ext]; dup {
		return
	}
	if ext-j.next > jbMaxJump { // stream discontinuity: release everything and restart
		j.flush(emit)
		j.next = ext
	}
	j.pkts[ext] = jbEntry{p, at}
	j.drain(at, emit)
}

func (j *jitterBuffer) drain(now time.Time, emit func(jbEntry, int64)) {
	for len(j.pkts) > 0 {
		if e, ok := j.pkts[j.next]; ok {
			delete(j.pkts, j.next)
			j.next++
			emit(e, 0)
			continue
		}
		first := int64(math.MaxInt64)
		for k := range j.pkts {
			if k < first {
				first = k
			}
		}
		e := j.pkts[first]
		if now.Sub(e.at) < j.delay && len(j.pkts) < jbMaxPackets {
			return
		}
		lost := first - j.next
		j.lost.Add(uint64(lost))
		delete(j.pkts, first)
		j.next = first + 1
		emit(e, lost)
	}
}

func (j *jitterBuffer) flush(emit func(jbEntry, int64)) {
	keys := make([]int64, 0, len(j.pkts))
	for k := range j.pkts {
		keys = append(keys, k)
	}
	sort.Slice(keys, func(a, b int) bool { return keys[a] < keys[b] })
	for _, k := range keys {
		lost := k - j.next
		j.lost.Add(uint64(lost))
		j.next = k + 1
		emit(j.pkts[k], lost)
		delete(j.pkts, k)
	}
}

// Kinds of muxer output, used by sinks that may need to drop data.
const (
	chunkHeader = iota
	chunkCluster
	chunkVideoKey
	chunkVideo
	chunkAudio
)

// sink receives the Matroska stream: a file (written synchronously) or a
// player/stdout pipe (written by a goroutine so a slow reader cannot block RTP).
type sink struct {
	name    string
	w       io.Writer
	ch      chan []byte // nil for synchronous sinks
	failed  atomic.Bool
	waitKey bool
	dropped uint64
	done    chan struct{}
}

func newAsyncSink(name string, w io.Writer) *sink {
	s := &sink{name: name, w: w, ch: make(chan []byte, 4096), done: make(chan struct{})}
	go func() {
		defer close(s.done)
		for b := range s.ch {
			if s.failed.Load() {
				continue
			}
			if _, err := s.w.Write(b); err != nil {
				s.failed.Store(true)
				logf("Media output: %s stopped reading (%v)", s.name, err)
			}
		}
	}()
	return s
}

func (s *sink) send(b []byte, kind int) {
	if s.failed.Load() {
		return
	}
	if s.ch == nil {
		if _, err := s.w.Write(b); err != nil {
			s.failed.Store(true)
			logf("ERROR: media output %s: %v", s.name, err)
		}
		return
	}
	switch kind {
	case chunkVideo:
		if s.waitKey {
			s.dropped++
			return
		}
	case chunkVideoKey:
		s.waitKey = false
	}
	select {
	case s.ch <- b:
		return
	default:
	}
	if kind == chunkHeader || kind == chunkCluster {
		select {
		case s.ch <- b:
		case <-time.After(time.Second):
			s.failed.Store(true)
			logf("Media output: %s is not reading its input; stopped feeding it", s.name)
		}
		return
	}
	s.dropped++
	if kind == chunkVideo || kind == chunkVideoKey {
		s.waitKey = true // the next video frames would reference a missing one
	}
}

func (s *sink) close() {
	if s.ch != nil {
		close(s.ch)
		select {
		case <-s.done:
		case <-time.After(2 * time.Second):
		}
	}
	if c, ok := s.w.(io.Closer); ok && s.w != os.Stdout {
		c.Close()
	}
}

// mediaOut turns received RTP into a live Matroska stream for a player and/or a file.
type mediaOut struct {
	mu           sync.Mutex
	closed       bool // guarded by mu; no output after close
	delay        time.Duration
	sinks        []*sink
	tracks       []*outTrack
	header       bool
	clusterOpen  bool
	clusterTs    int64
	firstAudioAt time.Time
	keyAt        time.Time // first queued video keyframe
	pending      []pendingFrame
	expectVideo  bool // the server answer sends video
	expectAudio  bool // the server answer sends audio
	pli          func(ssrc uint32)
	cmd          *exec.Cmd
	playerDone   chan struct{}
}

// pendingFrame waits for the stream header.
type pendingFrame struct {
	t    *outTrack
	ts   int64
	key  bool
	data []byte
}

const (
	maxPending        = 5000
	audioWaitAfterKey = 2 * time.Second // wait this long for an expected audio track
	videoWaitForKey   = 3 * time.Second // then start audio-only
)

// outTrack assembles complete frames for one track.
type outTrack struct {
	out       *mediaOut
	kind      string
	video     bool
	ssrc      uint32
	clock     float64
	channels  uint16
	supported bool
	num       uint64 // Matroska track number; 0 = not in the stream header

	jb        jitterBuffer
	tsStarted bool
	tsHigh    int64

	cur       []*rtp.Packet
	haveCur   bool
	curTs     int64
	curAt     time.Time
	curBroken bool

	waitKey   bool
	sps, pps  []byte
	anchorSet bool
	anchor    float64 // ms
	lastPLI   time.Time

	written, incomplete, skipped atomic.Uint64
	excluded                     uint64 // guarded by mediaOut.mu
	gotFrame                     bool   // guarded by mediaOut.mu
	excludedLogged               bool   // guarded by mediaOut.mu
}

func newMediaOut(delay time.Duration, pli func(uint32)) *mediaOut {
	return &mediaOut{delay: delay, pli: pli, playerDone: make(chan struct{})}
}

// addRecordFile records the stream to dir/whep-check-<UTC time>.mkv.
func (o *mediaOut) addRecordFile(dir string) error {
	if err := os.MkdirAll(dir, 0o755); err != nil {
		return fmt.Errorf("create record directory: %w", err)
	}
	p := filepath.Join(dir, "whep-check-"+time.Now().UTC().Format("20060102-150405")+".mkv")
	f, err := os.Create(p)
	if err != nil {
		return err
	}
	o.sinks = append(o.sinks, &sink{name: p, w: f})
	logf("Recording to %s", p)
	return nil
}

// addStdout writes the live stream to standard output for another program.
func (o *mediaOut) addStdout() {
	o.sinks = append(o.sinks, newAsyncSink("stdout", os.Stdout))
	logf("Writing the live Matroska stream to stdout (logs go to stderr)")
}

// ffInputArgs are the ffplay/ffmpeg input options for the live stream. They
// must not include "-fflags nobuffer": it discards the packets read while
// probing the format, which include the first keyframe, so decoding would
// start on frames whose references are missing.
var ffInputArgs = []string{"-flags", "low_delay", "-f", "matroska", "-i", "pipe:0"}

// startPlayer runs the player with the live stream on its standard input.
func (o *mediaOut) startPlayer(ctx context.Context, player string) error {
	args := []string{"-"}
	if strings.HasPrefix(strings.ToLower(filepath.Base(player)), "ffplay") {
		args = append([]string{"-hide_banner", "-loglevel", "warning", "-framedrop", "-autoexit",
			"-window_title", "whep-check"}, ffInputArgs...)
	}
	cmd := exec.CommandContext(ctx, player, args...)
	cmd.Stdout, cmd.Stderr = os.Stderr, os.Stderr
	stdin, err := cmd.StdinPipe()
	if err != nil {
		return err
	}
	if err := cmd.Start(); err != nil {
		return err
	}
	o.cmd = cmd
	o.sinks = append(o.sinks, newAsyncSink(filepath.Base(player), stdin))
	logf("Started %s (pid %d); it shows the picture after the first complete keyframe", player, cmd.Process.Pid)
	go func() {
		_ = cmd.Wait()
		close(o.playerDone)
	}()
	return nil
}

func (o *mediaOut) addTrack(kind, mime string, ssrc, clock uint32, channels uint16) *outTrack {
	t := &outTrack{
		out: o, kind: kind, video: kind == "video", ssrc: ssrc, clock: float64(clock), channels: channels,
		jb: jitterBuffer{delay: o.delay}, waitKey: kind == "video",
		supported: strings.EqualFold(mime, "video/H264") || strings.EqualFold(mime, "audio/opus"),
	}
	if !t.supported {
		logf("Media output: %s codec %s is not supported for -play/-record; the track is skipped", kind, mime)
	}
	o.mu.Lock()
	o.tracks = append(o.tracks, t)
	o.mu.Unlock()
	return t
}

// push takes a packet in arrival order.
func (t *outTrack) push(p *rtp.Packet, at time.Time) {
	if !t.supported {
		return
	}
	t.jb.push(p, at, t.ordered)
}

// ordered receives packets in sequence order; lost > 0 means packets were given up before p.
func (t *outTrack) ordered(e jbEntry, lost int64) {
	p := e.pkt
	var ts int64
	if !t.tsStarted {
		t.tsStarted = true
		ts = int64(p.Timestamp) + 1<<32
		t.tsHigh = ts
	} else {
		ts = t.tsHigh + int64(int32(p.Timestamp-uint32(t.tsHigh)))
		if ts > t.tsHigh {
			t.tsHigh = ts
		}
	}
	if t.haveCur && ts != t.curTs {
		t.finish(false) // the previous frame ended without its marker packet
	}
	if !t.haveCur {
		t.cur, t.haveCur, t.curTs, t.curAt, t.curBroken = t.cur[:0], true, ts, e.at, false
	}
	if lost > 0 {
		t.curBroken = true // a missing packet belonged to this frame or the one before it
	}
	t.cur = append(t.cur, p)
	if !t.video || p.Marker {
		t.finish(true)
	}
}

func (t *outTrack) finish(complete bool) {
	pkts, ts, at, broken := t.cur, t.curTs, t.curAt, t.curBroken
	t.haveCur = false
	if !t.video {
		for _, p := range pkts {
			t.out.writeFrame(t, t.tsMs(ts, at), true, p.Payload)
		}
		return
	}
	if !complete || broken {
		t.incomplete.Add(1)
		t.needKeyframe()
		return
	}
	dp := codecs.H264Packet{IsAVC: true}
	var data []byte
	for _, p := range pkts {
		b, err := dp.Unmarshal(p.Payload)
		if err != nil {
			t.incomplete.Add(1)
			t.needKeyframe()
			return
		}
		data = append(data, b...)
	}
	key := false
	for i := 0; i+4 <= len(data); {
		n := int(binary.BigEndian.Uint32(data[i:]))
		i += 4
		if n <= 0 || i+n > len(data) {
			break
		}
		switch data[i] & 0x1f {
		case 5:
			key = true
		case 7:
			t.sps = append(t.sps[:0], data[i:i+n]...)
		case 8:
			t.pps = append(t.pps[:0], data[i:i+n]...)
		}
		i += n
	}
	if len(data) == 0 {
		return
	}
	if t.waitKey && !key {
		t.skipped.Add(1)
		return
	}
	t.waitKey = false
	t.out.writeFrame(t, t.tsMs(ts, at), key, data)
}

// needKeyframe skips video until the next keyframe and asks the server for one.
func (t *outTrack) needKeyframe() {
	t.waitKey = true
	if t.out.pli != nil && time.Since(t.lastPLI) >= time.Second {
		t.lastPLI = time.Now()
		t.out.pli(t.ssrc)
	}
}

// tsMs maps an RTP timestamp to the output timeline, anchored at the arrival
// of the track's first written frame.
func (t *outTrack) tsMs(ts int64, at time.Time) int64 {
	rtpMs := float64(ts) * 1000 / t.clock
	if !t.anchorSet {
		t.anchorSet = true
		t.anchor = float64(at.Sub(start).Microseconds())/1000 - rtpMs
	}
	v := int64(math.Round(t.anchor + rtpMs))
	if v < 0 {
		v = 0
	}
	return v
}

func (o *mediaOut) emit(b []byte, kind int) {
	for _, s := range o.sinks {
		s.send(b, kind)
	}
}

func (o *mediaOut) writeFrame(t *outTrack, ts int64, key bool, data []byte) {
	o.mu.Lock()
	defer o.mu.Unlock()
	if o.closed {
		return // tracks can still deliver frames while the session shuts down
	}
	t.gotFrame = true
	if !o.header {
		if t.video && key && o.keyAt.IsZero() && len(t.sps) >= 4 && len(t.pps) > 0 {
			o.keyAt = time.Now()
		}
		if !t.video && o.firstAudioAt.IsZero() {
			o.firstAudioAt = time.Now()
		}
		if (t.video && o.keyAt.IsZero()) || len(o.pending) >= maxPending {
			return // video before the first keyframe cannot start the stream
		}
		o.pending = append(o.pending, pendingFrame{t, ts, key, append([]byte(nil), data...)})
		if !o.ready() {
			return
		}
		o.writeHeader()
		pending := o.pending
		o.pending = nil
		// Start audio with the first video keyframe (or shortly before now
		// for audio only), so queued audio does not add playback delay.
		cutoff := int64(time.Since(start)/time.Millisecond) - 300
		for _, f := range pending {
			if f.t.video {
				cutoff = f.ts
				break
			}
		}
		for _, f := range pending {
			if !f.t.video && f.ts < cutoff {
				continue
			}
			o.writeBlock(f.t, f.ts, f.key, f.data)
		}
		return
	}
	o.writeBlock(t, ts, key, data)
}

// ready reports whether the stream header can be written: a video keyframe
// is queued and the expected audio has started (or did not start in time).
func (o *mediaOut) ready() bool {
	var audio *outTrack
	for _, x := range o.tracks {
		if x.supported && !x.video && audio == nil {
			audio = x
		}
	}
	audioReady := audio != nil && audio.gotFrame
	if o.keyAt.IsZero() {
		if !audioReady {
			return false
		}
		// Audio only: start right away unless video is expected, then wait for it a while.
		return !o.expectVideo || time.Since(o.firstAudioAt) >= videoWaitForKey
	}
	return audioReady || !o.expectAudio || time.Since(o.keyAt) >= audioWaitAfterKey
}

func (o *mediaOut) writeBlock(t *outTrack, ts int64, key bool, data []byte) {
	if t.num == 0 {
		t.excluded++
		if !t.excludedLogged {
			t.excludedLogged = true
			logf("WARNING: media output: the %s track started after the stream header; it is not played or recorded", t.kind)
		}
		return
	}
	if !o.clusterOpen || (t.video && key) || ts-o.clusterTs > 5000 || ts-o.clusterTs < -30000 {
		o.clusterOpen, o.clusterTs = true, ts
		o.emit(mkvCluster(ts), chunkCluster)
	}
	kind := chunkAudio
	if t.video {
		kind = chunkVideo
		if key {
			kind = chunkVideoKey
		}
	}
	o.emit(mkvSimpleBlock(t.num, int16(ts-o.clusterTs), key, data), kind)
	t.written.Add(1)
}

// writeHeader writes the stream header for the tracks that have frames.
func (o *mediaOut) writeHeader() {
	var video, audio *outTrack
	for _, x := range o.tracks {
		if x.supported && x.video && video == nil && !o.keyAt.IsZero() {
			video = x
		}
		if x.supported && !x.video && audio == nil && x.gotFrame {
			audio = x
		}
	}
	var tracks []mkvTrack
	var desc []string
	if video != nil {
		w, h := 16, 16
		if info, err := parseSPS(video.sps); err == nil {
			w, h = info.Width, info.Height
		}
		video.num = 1
		tracks = append(tracks, mkvTrack{number: 1, video: true, codecID: "V_MPEG4/ISO/AVC",
			codecPrivate: avcDecoderConfig(video.sps, video.pps), width: w, height: h})
		desc = append(desc, fmt.Sprintf("video H.264 %dx%d", w, h))
	}
	if audio != nil {
		ch := audio.channels
		if ch == 0 {
			ch = 2
		}
		audio.num = 2
		tracks = append(tracks, mkvTrack{number: 2, codecID: "A_OPUS", codecPrivate: opusHead(uint8(ch), 48000),
			sampleRate: 48000, channels: uint64(ch)})
		desc = append(desc, fmt.Sprintf("audio Opus %d ch", ch))
	}
	o.emit(mkvHeader(tracks), chunkHeader)
	o.header = true
	logf("Media output: stream started with %s (jitter buffer %v)", strings.Join(desc, ", "), o.delay)
	if o.expectAudio && audio == nil {
		logf("WARNING: media output: no audio arrived within %v of the first keyframe; starting without audio", audioWaitAfterKey)
	}
}

// expect sets which media the server answer sends.
func (o *mediaOut) expect(video, audio bool) {
	o.mu.Lock()
	o.expectVideo, o.expectAudio = video, audio
	o.mu.Unlock()
}

// statsLine summarizes the output since the start.
func (o *mediaOut) statsLine() string {
	o.mu.Lock()
	defer o.mu.Unlock()
	var parts []string
	for _, t := range o.tracks {
		if !t.supported {
			continue
		}
		p := fmt.Sprintf("%s frames written %d", t.kind, t.written.Load())
		if t.video {
			p += fmt.Sprintf(", incomplete %d, skipped until keyframe %d", t.incomplete.Load(), t.skipped.Load())
		}
		p += fmt.Sprintf(", buffer gave up %d pkts, too late %d", t.jb.lost.Load(), t.jb.late.Load())
		if t.excluded > 0 {
			p += fmt.Sprintf(", not in stream header %d", t.excluded)
		}
		parts = append(parts, p)
	}
	for _, s := range o.sinks {
		if s.dropped > 0 {
			parts = append(parts, fmt.Sprintf("%s dropped %d frames (not reading fast enough)", s.name, s.dropped))
		}
	}
	if !o.header {
		parts = append(parts, "waiting for the first keyframe")
	}
	return "Output: " + strings.Join(parts, "; ")
}

func (o *mediaOut) close() {
	o.mu.Lock()
	o.closed = true
	o.mu.Unlock()
	for _, s := range o.sinks {
		s.close()
	}
	if o.cmd != nil && o.cmd.Process != nil {
		// The player sees the end of its input; give it a moment to exit by itself.
		select {
		case <-o.playerDone:
		case <-time.After(time.Second):
			_ = o.cmd.Process.Kill()
			<-o.playerDone
		}
	}
}
