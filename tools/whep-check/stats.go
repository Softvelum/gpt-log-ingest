package main

import (
	"bytes"
	"fmt"
	"math"
	"strings"
	"sync"
	"time"

	"github.com/pion/interceptor"
	"github.com/pion/rtcp"
	"github.com/pion/rtp"
)

const (
	seenWindow   = 8192 // sequence numbers remembered for duplicate detection
	minStallGap  = 250 * time.Millisecond
	stallLogGap  = time.Second // at most one stall message per track per second
	maxTimingGap = 5.0         // seconds; larger RTP steps are treated as discontinuities
)

// running accumulates a mean and standard deviation.
type running struct{ n, sum, sq float64 }

func (r *running) add(v float64) { r.n++; r.sum += v; r.sq += v * v }
func (r running) mean() float64 {
	if r.n == 0 {
		return 0
	}
	return r.sum / r.n
}
func (r running) std() float64 {
	if r.n < 2 {
		return 0
	}
	m := r.mean()
	return math.Sqrt(math.Max(0, r.sq/r.n-m*m))
}

// counters are the cumulative values used for per-interval deltas.
type counters struct {
	packets, bytes, lost, late, dup, frames, keyframes, nacked, stalls uint64
}

// trackStats analyzes the RTP packets of one track as they arrive from the network,
// before any buffering, so it shows what the server actually delivered.
type trackStats struct {
	mu sync.Mutex

	kind, codec string
	ssrc        uint32
	clock       float64
	fmtp        string
	channels    uint16
	isH264      bool
	isOpus      bool

	first, last    time.Time
	packets, bytes uint64

	// Sequence numbers.
	seqStarted        bool
	baseSeq, highSeq  int64
	seen              [seenWindow]int64
	unique, dup, late uint64

	// Frames (packets sharing an RTP timestamp, in arrival order).
	tsStarted    bool
	curTs        int64 // extended timestamp of the current frame
	highTs       int64 // highest extended timestamp seen
	frames       uint64
	reordered    bool // presentation timestamps go backwards in sequence order (B-frames)
	lastFrameAt  time.Time
	stepEMA      float64          // typical RTP step between frames, seconds
	rtpStep      running          // RTP timestamp step between frames, seconds
	stepHist     map[int64]uint64 // RTP steps in 0.1 ms units
	arrivalStep  running          // arrival interval between frames, seconds
	maxGap       time.Duration
	stalls       uint64
	stallDelayed uint64 // stalls where RTP time advanced much less than the wall clock
	lastStallLog time.Time
	transitMin   float64 // ms
	transitSet   bool
	maxLate      float64 // ms above the lowest transit time

	// Keyframes.
	keyframes      uint64
	haveKey        bool
	lastKeyTs      int64
	gop            running
	gopMin, gopMax float64

	// Media format.
	sps, pps    []byte
	format      *spsInfo
	entropy     string
	formatNotes []string
	opus        *opusTOC

	// Interval state.
	prev      counters
	prevAt    time.Time
	ivMaxGap  time.Duration
	ivMaxLate float64
	ivGop     running

	rtcp *rtcpCounter
}

func newTrackStats(kind, codec string, ssrc, clock uint32, channels uint16, fmtp string, rc *rtcpCounter) *trackStats {
	s := &trackStats{
		kind: kind, codec: codec, ssrc: ssrc, clock: float64(clock), channels: channels, fmtp: fmtp,
		isH264: strings.EqualFold(codec, "video/H264"),
		isOpus: strings.EqualFold(codec, "audio/opus"),
		rtcp:   rc, prevAt: time.Now(), stepHist: map[int64]uint64{},
	}
	if s.clock == 0 {
		s.clock = 90000
	}
	for i := range s.seen {
		s.seen[i] = -1
	}
	return s
}

func (s *trackStats) label() string {
	return s.kind + " " + strings.TrimPrefix(strings.TrimPrefix(s.codec, "video/"), "audio/")
}

// onPacket records one received RTP packet.
func (s *trackStats) onPacket(p *rtp.Packet, now time.Time) {
	s.mu.Lock()
	defer s.mu.Unlock()

	if s.packets == 0 {
		s.first = now
		logf("First %s RTP packet received (seq=%d ts=%d)", s.kind, p.SequenceNumber, p.Timestamp)
	}
	s.packets++
	s.bytes += uint64(len(p.Payload))
	s.last = now

	// Sequence-number accounting.
	var ext int64
	newest := false
	if !s.seqStarted {
		s.seqStarted = true
		ext = int64(p.SequenceNumber) + 1<<32 // keep extended numbers positive
		s.baseSeq, s.highSeq = ext, ext
		newest = true
	} else {
		ext = s.highSeq + int64(int16(p.SequenceNumber-uint16(s.highSeq)))
	}
	switch {
	case ext < s.baseSeq || s.highSeq-ext >= seenWindow:
		s.late++
		return
	case s.seen[ext%seenWindow] == ext:
		s.dup++
		return
	}
	s.seen[ext%seenWindow] = ext
	s.unique++
	if ext < s.highSeq {
		s.late++ // reordered or retransmitted after a NACK
	}
	if ext > s.highSeq {
		s.highSeq = ext
		newest = true
	}

	// Timestamp unwrapping.
	var ts int64
	if !s.tsStarted {
		s.tsStarted = true
		ts = int64(p.Timestamp) + 1<<32
		s.curTs, s.highTs = ts, ts
		s.newFrame(ts, now, true)
	} else {
		ts = s.highTs + int64(int32(p.Timestamp-uint32(s.highTs)))
		if newest && ts != s.curTs {
			if ts < s.highTs {
				s.reordered = true
			}
			s.newFrame(ts, now, false)
			s.curTs = ts
		}
	}

	if s.isH264 {
		s.h264(p.Payload, ts)
	}
	if s.isOpus && s.opus == nil {
		if toc, err := parseOpusTOC(p.Payload); err == nil {
			s.opus = &toc
			logf("Audio format: Opus %d Hz, %d channel(s) negotiated; stream: %s", int(s.clock), s.channels, toc)
			if s.fmtp != "" {
				logf("  fmtp: %s", s.fmtp)
			}
		}
	}
}

// newFrame analyzes the arrival timing of a frame's first packet.
func (s *trackStats) newFrame(ts int64, now time.Time, first bool) {
	s.frames++
	if !first {
		gap := now.Sub(s.lastFrameAt)
		s.arrivalStep.add(gap.Seconds())
		if gap > s.maxGap {
			s.maxGap = gap
		}
		if gap > s.ivMaxGap {
			s.ivMaxGap = gap
		}
		adv := float64(ts-s.highTs) / s.clock // RTP time advanced since the newest earlier frame
		threshold := minStallGap
		if d := time.Duration(4 * s.stepEMA * float64(time.Second)); d > threshold {
			threshold = d
		}
		if gap > threshold {
			s.stalls++
			delayed := adv < 0.7*gap.Seconds()
			if delayed {
				s.stallDelayed++
			}
			if now.Sub(s.lastStallLog) >= stallLogGap {
				s.lastStallLog = now
				why := "the source paused or frames were dropped before packetization"
				if delayed {
					why = "frames were delayed in delivery and arrive late or in a burst"
				}
				logf("WARNING: %s gap: no new %s frame for %d ms, RTP time advanced %d ms: %s",
					s.kind, s.kind, gap.Milliseconds(), int64(math.Max(adv, 0)*1000), why)
			}
		}
		if ts > s.highTs {
			if step := float64(ts-s.highTs) / s.clock; step < maxTimingGap {
				s.rtpStep.add(step)
				s.stepHist[int64(math.Round(step*10000))]++
				if s.stepEMA == 0 {
					s.stepEMA = step
				} else {
					s.stepEMA += 0.1 * (step - s.stepEMA)
				}
			}
		}
	}
	if ts > s.highTs {
		s.highTs = ts
	}
	s.lastFrameAt = now

	transit := float64(now.Sub(start).Microseconds())/1000 - float64(ts)*1000/s.clock
	if !s.transitSet || transit < s.transitMin {
		s.transitMin, s.transitSet = transit, true
	}
	late := transit - s.transitMin
	if late > s.maxLate {
		s.maxLate = late
	}
	if late > s.ivMaxLate {
		s.ivMaxLate = late
	}
}

// h264 counts keyframes once per frame and tracks parameter sets.
func (s *trackStats) h264(payload []byte, ts int64) {
	spsChanged := false
	for _, n := range h264PayloadNALs(payload) {
		switch n.typ {
		case 5:
			if !s.haveKey || ts > s.lastKeyTs {
				if s.haveKey {
					g := float64(ts-s.lastKeyTs) / s.clock
					s.gop.add(g)
					s.ivGop.add(g)
					if s.gopMin == 0 || g < s.gopMin {
						s.gopMin = g
					}
					if g > s.gopMax {
						s.gopMax = g
					}
				} else {
					logf("First H.264 keyframe (IDR) received")
				}
				s.haveKey, s.lastKeyTs = true, ts
				s.keyframes++
			}
		case 7:
			if n.data != nil && !bytes.Equal(n.data, s.sps) {
				s.sps = append([]byte(nil), n.data...)
				spsChanged = true
			}
		case 8:
			if n.data != nil && !bytes.Equal(n.data, s.pps) {
				s.pps = append([]byte(nil), n.data...)
				if e, err := parsePPSEntropy(n.data); err == nil {
					s.entropy = e
				}
			}
		}
	}
	if spsChanged {
		s.logFormat()
	}
}

func (s *trackStats) logFormat() {
	info, err := parseSPS(s.sps)
	if err != nil {
		logf("Video format: SPS could not be parsed: %v", err)
		return
	}
	prefix := "Video format"
	if s.format != nil {
		prefix = fmt.Sprintf("Video format changed (%dx%d -> %dx%d)", s.format.Width, s.format.Height, info.Width, info.Height)
	}
	s.format = info
	lines := info.describe(s.entropy)
	logf("%s: %s", prefix, lines[0])
	for _, l := range lines[1:] {
		logf("  %s", l)
	}
	if s.fmtp != "" {
		logf("  fmtp: %s", s.fmtp)
	}
	for _, n := range s.checkFormat(info.FPS()) {
		logf("  %s", n)
	}
}

// checkFormat compares the SPS with the SDP and the H.264 level limits.
func (s *trackStats) checkFormat(fps float64) []string {
	var notes []string
	if s.format == nil {
		return nil
	}
	f := s.format
	if plid := fmtpValue(s.fmtp, "profile-level-id"); len(plid) == 6 {
		sdp := describeProfileLevelID(plid)
		sps := h264ProfileName(f.Profile, f.Constraints) + " " + h264LevelName(f.Profile, f.Constraints, f.Level)
		if sdp != "" && sdp != sps {
			notes = append(notes, fmt.Sprintf("NOTE: SDP profile-level-id %s (%s) differs from the stream SPS (%s)", plid, sdp, sps))
		}
	}
	if w := levelCheck(f, fps); w != "" {
		notes = append(notes, "WARNING: "+w)
	}
	return notes
}

func (s *trackStats) snapshot() counters {
	c := counters{packets: s.packets, bytes: s.bytes, late: s.late, dup: s.dup,
		frames: s.frames, keyframes: s.keyframes, stalls: s.stalls}
	if s.seqStarted {
		if exp := uint64(s.highSeq - s.baseSeq + 1); exp > s.unique {
			c.lost = exp - s.unique
		}
	}
	if s.rtcp != nil {
		c.nacked, _ = s.rtcp.get(s.ssrc)
	}
	return c
}

func delta(cur, prev uint64) int64 { return int64(cur) - int64(prev) }

// intervalLine reports the period since the previous call and starts a new one.
func (s *trackStats) intervalLine(now time.Time) string {
	s.mu.Lock()
	defer s.mu.Unlock()
	c := s.snapshot()
	p := s.prev
	dt := now.Sub(s.prevAt).Seconds()
	s.prev, s.prevAt = c, now
	defer func() { s.ivMaxGap, s.ivMaxLate, s.ivGop = 0, 0, running{} }()

	if c.packets == p.packets {
		idle := "never"
		if !s.last.IsZero() {
			idle = fmt.Sprintf("%.1f s ago", now.Sub(s.last).Seconds())
		}
		return fmt.Sprintf("Stats %s: no packets in this interval (last packet %s)", s.label(), idle)
	}
	kbps := float64(c.bytes-p.bytes) * 8 / dt / 1000
	line := fmt.Sprintf("Stats %s: %.0f kbps, packets +%d, lost %d (%+d), late %d (+%d), dup %d (+%d), NACKed %d (+%d); frames +%d (%.1f/s)",
		s.label(), kbps, c.packets-p.packets, c.lost, delta(c.lost, p.lost), c.late, c.late-p.late,
		c.dup, c.dup-p.dup, c.nacked, c.nacked-p.nacked, c.frames-p.frames, float64(c.frames-p.frames)/dt)
	if s.isH264 {
		line += fmt.Sprintf(", keyframes +%d", c.keyframes-p.keyframes)
		if s.ivGop.n > 0 {
			line += fmt.Sprintf(" (GOP %.2f s)", s.ivGop.mean())
		}
	}
	line += fmt.Sprintf(", max frame gap %d ms, max delay +%.0f ms", s.ivMaxGap.Milliseconds(), s.ivMaxLate)
	if n := c.stalls - p.stalls; n > 0 {
		line += fmt.Sprintf(" [%d gap(s) > %d ms]", n, minStallGap.Milliseconds())
	}
	return line
}

// finalLines summarizes the whole session for this track.
func (s *trackStats) finalLines() []string {
	s.mu.Lock()
	defer s.mu.Unlock()
	if s.packets == 0 {
		return []string{fmt.Sprintf("Final %s: no packets received", s.label())}
	}
	c := s.snapshot()
	dur := s.last.Sub(s.first).Seconds()
	var kbps, fps float64
	if dur > 0 {
		kbps = float64(c.bytes) * 8 / dur / 1000
		fps = float64(c.frames-1) / dur
	}
	exp := c.lost + s.unique
	lostPct := 0.0
	if exp > 0 {
		lostPct = 100 * float64(c.lost) / float64(exp)
	}
	var pli uint64
	if s.rtcp != nil {
		_, pli = s.rtcp.get(s.ssrc)
	}
	lines := []string{fmt.Sprintf("Final %s: %.1f s, %.0f kbps avg, packets %d, lost %d (%.3f%%), late %d, dup %d, NACKed %d, PLI sent %d",
		s.label(), dur, kbps, c.packets, c.lost, lostPct, c.late, c.dup, c.nacked, pli)}

	l := fmt.Sprintf("  frames %d (%.2f/s avg)", c.frames, fps)
	if s.isH264 {
		l += fmt.Sprintf(", keyframes %d", c.keyframes)
		if s.gop.n > 0 {
			l += fmt.Sprintf(" (GOP %.2f s avg, %.2f-%.2f s)", s.gop.mean(), s.gopMin, s.gopMax)
		}
		if s.reordered {
			l += ", B-frames: yes (timestamps reordered)"
		} else {
			l += ", B-frames: none seen"
		}
	}
	if s.opus != nil {
		l += fmt.Sprintf(", Opus %s %s %v", s.opus.Mode, s.opus.Bandwidth, s.opus.PacketDuration())
	}
	lines = append(lines, l)
	mode, offGrid := s.stepGrid()
	lines = append(lines, fmt.Sprintf("  timing: RTP step %.1f ms (%.1f%% off that grid), arrival step %.1f ± %.1f ms, max gap %d ms, gaps > %d ms: %d (%d delayed in delivery), max delay +%.0f ms",
		mode*1000, offGrid*100, s.arrivalStep.mean()*1000, s.arrivalStep.std()*1000,
		s.maxGap.Milliseconds(), minStallGap.Milliseconds(), s.stalls, s.stallDelayed, s.maxLate))
	if s.format != nil {
		desc := s.format.describe(s.entropy)
		lines = append(lines, "  format: "+desc[0])
		for _, n := range s.checkFormat(fps) {
			if strings.HasPrefix(n, "WARNING:") {
				n += " (measured frame rate)"
			}
			lines = append(lines, "  "+n)
		}
	}
	if v := s.verdict(); v != "" {
		lines = append(lines, "  "+v)
	}
	return lines
}

// stepGrid returns the most common RTP step between frames (seconds) and the
// share of steps that are not a whole multiple of it. Lost frames are whole
// multiples, so they do not count as irregular timing.
func (s *trackStats) stepGrid() (mode, offGrid float64) {
	var best int64
	var bestN, total, off uint64
	for k, n := range s.stepHist {
		total += n
		if n > bestN || (n == bestN && k < best) {
			best, bestN = k, n
		}
	}
	if best <= 0 || total == 0 {
		return 0, 0
	}
	for k, n := range s.stepHist {
		r := float64(k) / float64(best)
		if math.Abs(r-math.Round(r)) > 0.15 || math.Round(r) == 0 {
			off += n
		}
	}
	return float64(best) / 10000, float64(off) / float64(total)
}

// verdict interprets the timing figures in one line.
func (s *trackStats) verdict() string {
	if s.stalls == 0 || s.rtpStep.n < 10 {
		return ""
	}
	_, offGrid := s.stepGrid()
	irregular := ""
	if offGrid > 0.05 {
		irregular = fmt.Sprintf("; also %.0f%% of RTP steps are off the frame grid: the source frame timing is uneven", offGrid*100)
	}
	if s.stallDelayed*2 >= s.stalls {
		return "TIMING: frames arrived in bursts after gaps while their RTP timestamps continued: delivery was delayed somewhere between the source clock and this client (source ingest, server or network), not lost" + irregular
	}
	return "TIMING: RTP time advanced together with the gaps: the source sent no frames during them (encoder/source pauses or frames dropped before WebRTC)" + irregular
}

// rtcpCounter counts the NACK and PLI requests this client sends, per media SSRC.
type rtcpCounter struct {
	mu     sync.Mutex
	nacked map[uint32]uint64 // sequence numbers requested
	plis   map[uint32]uint64
}

func newRTCPCounter() *rtcpCounter {
	return &rtcpCounter{nacked: map[uint32]uint64{}, plis: map[uint32]uint64{}}
}

func (c *rtcpCounter) get(ssrc uint32) (nacked, plis uint64) {
	c.mu.Lock()
	defer c.mu.Unlock()
	return c.nacked[ssrc], c.plis[ssrc]
}

func (c *rtcpCounter) observe(pkts []rtcp.Packet) {
	c.mu.Lock()
	defer c.mu.Unlock()
	for _, p := range pkts {
		switch p := p.(type) {
		case *rtcp.TransportLayerNack:
			for _, pair := range p.Nacks {
				c.nacked[p.MediaSSRC] += uint64(len(pair.PacketList()))
			}
		case *rtcp.PictureLossIndication:
			c.plis[p.MediaSSRC]++
		}
	}
}

// NewInterceptor implements interceptor.Factory.
func (c *rtcpCounter) NewInterceptor(string) (interceptor.Interceptor, error) {
	return &rtcpCountInterceptor{c: c}, nil
}

type rtcpCountInterceptor struct {
	interceptor.NoOp
	c *rtcpCounter
}

// BindRTCPWriter observes outgoing RTCP. Registered before the NACK generator,
// so the generator's requests pass through this writer too.
func (i *rtcpCountInterceptor) BindRTCPWriter(w interceptor.RTCPWriter) interceptor.RTCPWriter {
	return interceptor.RTCPWriterFunc(func(pkts []rtcp.Packet, a interceptor.Attributes) (int, error) {
		i.c.observe(pkts)
		return w.Write(pkts, a)
	})
}
