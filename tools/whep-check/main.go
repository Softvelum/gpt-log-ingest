// whep-check is a WHEP playback client for diagnosing WebRTC delivery.
//
// It performs the WHEP signaling exchange, reports the server answer
// (ICE mode, candidates, codecs), follows ICE/DTLS progress, counts the
// received RTP media and can optionally save or play the stream.
package main

import (
	"bytes"
	"context"
	"errors"
	"flag"
	"fmt"
	"io"
	"net"
	"net/http"
	"net/url"
	"os"
	"os/exec"
	"os/signal"
	"path/filepath"
	"strings"
	"sync"
	"sync/atomic"
	"syscall"
	"time"

	"github.com/pion/interceptor"
	"github.com/pion/logging"
	"github.com/pion/rtcp"
	"github.com/pion/rtp"
	"github.com/pion/webrtc/v4"
	"github.com/pion/webrtc/v4/pkg/media/h264writer"
	"github.com/pion/webrtc/v4/pkg/media/oggwriter"
)

// Exit codes.
const (
	exitOK        = 0
	exitUsage     = 1
	exitSignaling = 2
	exitICE       = 3
	exitNoMedia   = 4
)

type options struct {
	url         string
	token       string
	duration    time.Duration
	iceTimeout  time.Duration
	stun        string
	play        bool
	player      string
	playVideo   int
	playAudio   int
	recordDir   string
	verbose     bool
	insecureTLS bool

	// includeLoopback lets the in-process tests connect over 127.0.0.1.
	includeLoopback bool
}

type trackStats struct {
	kind      string
	codec     string
	ssrc      uint32
	packets   atomic.Uint64
	bytes     atomic.Uint64
	keyframes atomic.Uint64
	first     atomic.Int64 // unix nanos of the first packet
	last      atomic.Int64 // unix nanos of the latest packet
}

var start = time.Now()

func logf(format string, args ...any) {
	fmt.Printf("[%s +%6.2fs] %s\n", time.Now().UTC().Format("15:04:05.000"),
		time.Since(start).Seconds(), fmt.Sprintf(format, args...))
}

func main() {
	os.Exit(run())
}

func run() int {
	var o options
	flag.StringVar(&o.token, "token", "", "optional Bearer token for the WHEP endpoint")
	flag.DurationVar(&o.duration, "duration", 20*time.Second, "how long to receive media after connecting (0 = until Ctrl+C; default with -play)")
	flag.DurationVar(&o.iceTimeout, "ice-timeout", 15*time.Second, "fail if ICE/DTLS is not connected within this time")
	flag.StringVar(&o.stun, "stun", "", "optional STUN server, e.g. stun:stun.l.google.com:19302 (not needed for ice-lite servers)")
	flag.BoolVar(&o.play, "play", false, "forward media to a local player (ffplay) via RTP + SDP file")
	flag.StringVar(&o.player, "player", "ffplay", "player executable used with -play; empty = only write the SDP file")
	flag.IntVar(&o.playVideo, "play-video-port", 5004, "local UDP port for forwarded video RTP")
	flag.IntVar(&o.playAudio, "play-audio-port", 5006, "local UDP port for forwarded audio RTP")
	flag.StringVar(&o.recordDir, "record", "", "directory to save video.h264 and audio.ogg")
	flag.BoolVar(&o.verbose, "v", false, "verbose ICE/DTLS logging from the WebRTC stack")
	flag.BoolVar(&o.insecureTLS, "k", false, "skip TLS certificate verification for the WHEP HTTP requests")
	flag.Usage = func() {
		fmt.Fprintf(os.Stderr, "Usage: %s [options] <whep-url>\n\nOptions:\n", filepath.Base(os.Args[0]))
		flag.PrintDefaults()
		fmt.Fprintf(os.Stderr, "\nExit codes: 0 media received, 1 usage, 2 signaling failed, 3 ICE/DTLS failed, 4 connected but no media\n")
	}
	flag.Parse()
	if flag.NArg() != 1 {
		flag.Usage()
		return exitUsage
	}
	o.url = flag.Arg(0)
	durationSet := false
	flag.Visit(func(f *flag.Flag) { durationSet = durationSet || f.Name == "duration" })
	if o.play && !durationSet {
		o.duration = 0 // watch until Ctrl+C
	}

	ctx, cancel := signal.NotifyContext(context.Background(), os.Interrupt, syscall.SIGTERM)
	defer cancel()

	return check(ctx, o)
}

func check(ctx context.Context, o options) int {
	pc, err := newPeerConnection(o)
	if err != nil {
		logf("ERROR: create peer connection: %v", err)
		return exitUsage
	}
	defer pc.Close()

	for _, kind := range []webrtc.RTPCodecType{webrtc.RTPCodecTypeVideo, webrtc.RTPCodecTypeAudio} {
		if _, err := pc.AddTransceiverFromKind(kind, webrtc.RTPTransceiverInit{Direction: webrtc.RTPTransceiverDirectionRecvonly}); err != nil {
			logf("ERROR: add %s transceiver: %v", kind, err)
			return exitUsage
		}
	}

	connected := make(chan struct{})
	failed := make(chan string, 1)
	var connectedOnce sync.Once

	pc.OnICEConnectionStateChange(func(s webrtc.ICEConnectionState) {
		logf("ICE state: %s", s)
		switch s {
		case webrtc.ICEConnectionStateConnected:
			logSelectedPair(pc)
		case webrtc.ICEConnectionStateFailed:
			select {
			case failed <- "ICE failed":
			default:
			}
		}
	})
	pc.OnConnectionStateChange(func(s webrtc.PeerConnectionState) {
		logf("Peer connection state: %s", s)
		switch s {
		case webrtc.PeerConnectionStateConnected:
			connectedOnce.Do(func() { close(connected) })
		case webrtc.PeerConnectionStateFailed:
			select {
			case failed <- "peer connection failed (ICE or DTLS)":
			default:
			}
		}
	})

	// Media sinks.
	var (
		statsMu sync.Mutex
		stats   []*trackStats
	)
	var fwd atomic.Pointer[forwarder]
	var rec *recorder
	if o.recordDir != "" {
		if rec, err = newRecorder(o.recordDir); err != nil {
			logf("ERROR: %v", err)
			return exitUsage
		}
		defer rec.close()
	}

	pc.OnTrack(func(track *webrtc.TrackRemote, _ *webrtc.RTPReceiver) {
		c := track.Codec()
		st := &trackStats{kind: track.Kind().String(), codec: c.MimeType, ssrc: uint32(track.SSRC())}
		statsMu.Lock()
		stats = append(stats, st)
		statsMu.Unlock()
		logf("Track: %s %s pt=%d clock=%d ssrc=%d fmtp=%q", st.kind, c.MimeType, c.PayloadType, c.ClockRate, st.ssrc, c.SDPFmtpLine)

		isVideo := track.Kind() == webrtc.RTPCodecTypeVideo
		isH264 := strings.EqualFold(c.MimeType, webrtc.MimeTypeH264)
		if isVideo {
			go requestKeyframes(ctx, pc, st.ssrc)
		}
		for {
			pkt, _, err := track.ReadRTP()
			if err != nil {
				if !errors.Is(err, io.EOF) {
					logf("%s track read ended: %v", st.kind, err)
				}
				return
			}
			now := time.Now().UnixNano()
			if st.packets.Add(1) == 1 {
				st.first.Store(now)
				logf("First %s RTP packet received (seq=%d ts=%d)", st.kind, pkt.SequenceNumber, pkt.Timestamp)
			}
			st.last.Store(now)
			st.bytes.Add(uint64(len(pkt.Payload)))
			if isH264 && h264Keyframe(pkt.Payload) {
				if st.keyframes.Add(1) == 1 {
					logf("First H.264 keyframe (IDR/SPS) received")
				}
			}
			if f := fwd.Load(); f != nil {
				f.write(isVideo, pkt)
			}
			if rec != nil {
				rec.write(isVideo, c.MimeType, pkt)
			}
		}
	})

	offer, err := pc.CreateOffer(nil)
	if err != nil {
		logf("ERROR: create offer: %v", err)
		return exitUsage
	}
	gathered := webrtc.GatheringCompletePromise(pc)
	if err := pc.SetLocalDescription(offer); err != nil {
		logf("ERROR: set local description: %v", err)
		return exitUsage
	}
	<-gathered
	local := pc.LocalDescription()
	logf("Local ICE candidates:")
	printCandidates(local.SDP)

	// WHEP signaling.
	logf("POST %s", o.url)
	answerSDP, location, err := postOffer(ctx, o, local.SDP)
	if err != nil {
		logf("ERROR: WHEP signaling: %v", err)
		return exitSignaling
	}
	if location != "" {
		logf("WHEP resource: %s", location)
		defer deleteResource(o, location)
	} else {
		logf("WARNING: no Location header in the answer; session cannot be deleted explicitly")
	}
	describeAnswer(answerSDP)

	if err := pc.SetRemoteDescription(webrtc.SessionDescription{Type: webrtc.SDPTypeAnswer, SDP: answerSDP}); err != nil {
		logf("ERROR: set remote description: %v", err)
		return exitSignaling
	}

	if o.play {
		f, err := startPlayback(ctx, o, pc)
		if err != nil {
			logf("ERROR: playback setup: %v", err)
			return exitUsage
		}
		fwd.Store(f)
		defer f.close()
	}

	// Wait for ICE + DTLS.
	timer := time.NewTimer(o.iceTimeout)
	defer timer.Stop()
	select {
	case <-connected:
		logf("Connected (ICE + DTLS) in %.2fs after start", time.Since(start).Seconds())
	case reason := <-failed:
		logf("FAILED: %s", reason)
		printICEHint(answerSDP)
		return exitICE
	case <-timer.C:
		logf("FAILED: not connected within %s (last ICE state: %s)", o.iceTimeout, pc.ICEConnectionState())
		printICEHint(answerSDP)
		return exitICE
	case <-ctx.Done():
		logf("Interrupted before connecting")
		return exitICE
	}

	// Receive media.
	var deadline <-chan time.Time
	if o.duration > 0 {
		logf("Receiving media for %s (Ctrl+C to stop earlier)", o.duration)
		deadline = time.After(o.duration)
	} else {
		logf("Receiving media until Ctrl+C")
	}
	ticker := time.NewTicker(5 * time.Second)
	defer ticker.Stop()
	snapshot := func() []*trackStats {
		statsMu.Lock()
		defer statsMu.Unlock()
		return append([]*trackStats(nil), stats...)
	}
loop:
	for {
		select {
		case <-ticker.C:
			printStats(snapshot(), false)
		case reason := <-failed:
			logf("Connection lost: %s", reason)
			break loop
		case <-deadline:
			break loop
		case <-ctx.Done():
			break loop
		}
	}

	all := snapshot()
	printStats(all, true)
	var total uint64
	for _, st := range all {
		total += st.packets.Load()
	}
	if total == 0 {
		logf("RESULT: connected, but no RTP media received")
		return exitNoMedia
	}
	logf("RESULT: OK, media received")
	return exitOK
}

func newPeerConnection(o options) (*webrtc.PeerConnection, error) {
	m := &webrtc.MediaEngine{}
	if err := m.RegisterDefaultCodecs(); err != nil {
		return nil, err
	}
	ir := &interceptor.Registry{}
	if err := webrtc.RegisterDefaultInterceptors(m, ir); err != nil {
		return nil, err
	}
	se := webrtc.SettingEngine{}
	lf := logging.NewDefaultLoggerFactory()
	if o.verbose {
		lf.ScopeLevels["ice"] = logging.LogLevelDebug
		lf.ScopeLevels["dtls"] = logging.LogLevelDebug
		lf.ScopeLevels["pc"] = logging.LogLevelDebug
	}
	se.LoggerFactory = lf
	if o.includeLoopback {
		se.SetIncludeLoopbackCandidate(true)
		se.SetNetworkTypes([]webrtc.NetworkType{webrtc.NetworkTypeUDP4})
	}
	api := webrtc.NewAPI(webrtc.WithMediaEngine(m), webrtc.WithInterceptorRegistry(ir), webrtc.WithSettingEngine(se))

	cfg := webrtc.Configuration{}
	if o.stun != "" {
		cfg.ICEServers = []webrtc.ICEServer{{URLs: []string{o.stun}}}
	}
	return api.NewPeerConnection(cfg)
}

func httpClient(o options) *http.Client {
	tr := http.DefaultTransport.(*http.Transport).Clone()
	if o.insecureTLS {
		tr.TLSClientConfig.InsecureSkipVerify = true
	}
	return &http.Client{Transport: tr, Timeout: 15 * time.Second}
}

func postOffer(ctx context.Context, o options, offer string) (answer, location string, err error) {
	req, err := http.NewRequestWithContext(ctx, http.MethodPost, o.url, strings.NewReader(offer))
	if err != nil {
		return "", "", err
	}
	req.Header.Set("Content-Type", "application/sdp")
	req.Header.Set("Accept", "application/sdp")
	if o.token != "" {
		req.Header.Set("Authorization", "Bearer "+o.token)
	}
	t0 := time.Now()
	resp, err := httpClient(o).Do(req)
	if err != nil {
		return "", "", err
	}
	defer resp.Body.Close()
	body, err := io.ReadAll(io.LimitReader(resp.Body, 1<<20))
	if err != nil {
		return "", "", err
	}
	logf("HTTP %s in %dms (proto %s, server %q, content-type %q)", resp.Status,
		time.Since(t0).Milliseconds(), resp.Proto, resp.Header.Get("Server"), resp.Header.Get("Content-Type"))
	if resp.StatusCode != http.StatusCreated && resp.StatusCode != http.StatusOK {
		return "", "", fmt.Errorf("unexpected status %s: %s", resp.Status, strings.TrimSpace(string(body)))
	}
	if !bytes.HasPrefix(bytes.TrimSpace(body), []byte("v=")) {
		return "", "", fmt.Errorf("response body is not an SDP answer: %.200q", body)
	}
	if loc := resp.Header.Get("Location"); loc != "" {
		base, _ := url.Parse(o.url)
		if ref, err := url.Parse(loc); err == nil && base != nil {
			location = base.ResolveReference(ref).String()
		} else {
			location = loc
		}
	}
	return string(body), location, nil
}

func deleteResource(o options, location string) {
	req, err := http.NewRequest(http.MethodDelete, location, nil)
	if err != nil {
		return
	}
	if o.token != "" {
		req.Header.Set("Authorization", "Bearer "+o.token)
	}
	resp, err := httpClient(o).Do(req)
	if err != nil {
		logf("DELETE %s: %v", location, err)
		return
	}
	resp.Body.Close()
	logf("DELETE WHEP resource: HTTP %s", resp.Status)
}

func sdpLines(sdp string) []string {
	return strings.Split(strings.ReplaceAll(sdp, "\r\n", "\n"), "\n")
}

func printCandidates(sdp string) {
	n := 0
	for _, l := range sdpLines(sdp) {
		if strings.HasPrefix(l, "a=candidate:") {
			logf("  %s", strings.TrimPrefix(l, "a="))
			n++
		}
	}
	if n == 0 {
		logf("  (none)")
	}
}

// describeAnswer prints the parts of the server answer relevant to connectivity.
func describeAnswer(sdp string) {
	logf("Server answer:")
	seen := map[string]bool{}
	for _, l := range sdpLines(sdp) {
		switch {
		case l == "a=ice-lite":
			logf("  ice-lite: yes (server only answers connectivity checks; client must reach it)")
		case strings.HasPrefix(l, "m="):
			logf("  %s", l)
		case strings.HasPrefix(l, "a=rtpmap:"), strings.HasPrefix(l, "a=setup:"):
			logf("    %s", strings.TrimPrefix(l, "a="))
		case strings.HasPrefix(l, "a=candidate:"):
			if seen[l] {
				continue
			}
			seen[l] = true
			logf("    %s", strings.TrimPrefix(l, "a="))
			if ip := candidateIP(l); ip != nil && (ip.IsPrivate() || ip.IsLoopback()) {
				logf("    WARNING: candidate address %s is not public; remote clients cannot reach it (NAT1To1IPs?)", ip)
			}
		}
	}
}

// candidateIP returns the connection address of an a=candidate line.
func candidateIP(line string) net.IP {
	f := strings.Fields(strings.TrimPrefix(line, "a="))
	if len(f) < 6 {
		return nil
	}
	return net.ParseIP(f[4])
}

func printICEHint(answer string) {
	var targets []string
	for _, l := range sdpLines(answer) {
		f := strings.Fields(strings.TrimPrefix(l, "a="))
		if strings.HasPrefix(l, "a=candidate:") && len(f) >= 6 {
			t := fmt.Sprintf("%s %s:%s", strings.ToLower(f[2]), f[4], f[5])
			if !contains(targets, t) {
				targets = append(targets, t)
			}
		}
	}
	logf("HINT: signaling worked but no ICE/DTLS connection. Check that %s is reachable from this client:",
		strings.Join(targets, ", "))
	logf("      server firewall / provider security group must allow inbound UDP to that port (range),")
	logf("      and this client network must allow outbound UDP. Re-run with -v for ICE details.")
}

func contains(s []string, v string) bool {
	for _, x := range s {
		if x == v {
			return true
		}
	}
	return false
}

func logSelectedPair(pc *webrtc.PeerConnection) {
	for _, t := range pc.GetTransceivers() {
		r := t.Receiver()
		if r == nil || r.Transport() == nil {
			continue
		}
		pair, err := r.Transport().ICETransport().GetSelectedCandidatePair()
		if err == nil && pair != nil {
			logf("Selected candidate pair: local %s %s:%d <-> remote %s %s:%d",
				pair.Local.Typ, pair.Local.Address, pair.Local.Port,
				pair.Remote.Typ, pair.Remote.Address, pair.Remote.Port)
			return
		}
	}
}

func requestKeyframes(ctx context.Context, pc *webrtc.PeerConnection, ssrc uint32) {
	t := time.NewTicker(2 * time.Second)
	defer t.Stop()
	for {
		if err := pc.WriteRTCP([]rtcp.Packet{&rtcp.PictureLossIndication{MediaSSRC: ssrc}}); err != nil {
			return
		}
		select {
		case <-ctx.Done():
			return
		case <-t.C:
		}
	}
}

// h264Keyframe reports whether an H.264 RTP payload carries an IDR slice or SPS.
func h264Keyframe(p []byte) bool {
	if len(p) < 2 {
		return false
	}
	isKey := func(t byte) bool { return t == 5 || t == 7 }
	switch t := p[0] & 0x1f; t {
	case 24: // STAP-A
		for i := 1; i+2 < len(p); {
			size := int(p[i])<<8 | int(p[i+1])
			if i+2 >= len(p) || size == 0 {
				break
			}
			if isKey(p[i+2] & 0x1f) {
				return true
			}
			i += 2 + size
		}
		return false
	case 28: // FU-A
		return p[1]&0x80 != 0 && isKey(p[1]&0x1f)
	default:
		return isKey(t)
	}
}

func printStats(all []*trackStats, final bool) {
	label := "Stats"
	if final {
		label = "Final stats"
	}
	if len(all) == 0 {
		logf("%s: no tracks yet", label)
		return
	}
	for _, st := range all {
		pk, by := st.packets.Load(), st.bytes.Load()
		var kbps float64
		if f, l := st.first.Load(), st.last.Load(); pk > 1 && l > f {
			kbps = float64(by*8) / (float64(l-f) / 1e9) / 1000
		}
		extra := ""
		if st.kind == "video" {
			extra = fmt.Sprintf(" keyframes=%d", st.keyframes.Load())
		}
		idle := ""
		if l := st.last.Load(); l > 0 {
			idle = fmt.Sprintf(" last=%.1fs ago", time.Since(time.Unix(0, l)).Seconds())
		}
		logf("%s: %s %s packets=%d payload=%dB avg=%.0fkbps%s%s", label, st.kind, st.codec, pk, by, kbps, extra, idle)
	}
}

// forwarder relays decrypted RTP to local UDP ports for an external player.
type forwarder struct {
	video, audio net.Conn
	cmd          *exec.Cmd
}

func startPlayback(ctx context.Context, o options, pc *webrtc.PeerConnection) (*forwarder, error) {
	var b strings.Builder
	b.WriteString("v=0\r\no=- 0 0 IN IP4 127.0.0.1\r\ns=whep-check\r\nc=IN IP4 127.0.0.1\r\nt=0 0\r\n")
	f := &forwarder{}
	for _, t := range pc.GetTransceivers() {
		codecs := t.Receiver().GetParameters().Codecs
		if len(codecs) == 0 {
			continue
		}
		c := codecs[0]
		kind, port := "audio", o.playAudio
		if t.Kind() == webrtc.RTPCodecTypeVideo {
			kind, port = "video", o.playVideo
		}
		conn, err := net.Dial("udp", fmt.Sprintf("127.0.0.1:%d", port))
		if err != nil {
			return nil, err
		}
		if kind == "video" {
			f.video = conn
		} else {
			f.audio = conn
		}
		enc := strings.TrimPrefix(c.MimeType, kind+"/")
		fmt.Fprintf(&b, "m=%s %d RTP/AVP %d\r\na=rtpmap:%d %s/%d", kind, port, c.PayloadType, c.PayloadType, enc, c.ClockRate)
		if c.Channels > 0 {
			fmt.Fprintf(&b, "/%d", c.Channels)
		}
		b.WriteString("\r\n")
		if c.SDPFmtpLine != "" {
			fmt.Fprintf(&b, "a=fmtp:%d %s\r\n", c.PayloadType, c.SDPFmtpLine)
		}
	}
	sdpPath := filepath.Join(os.TempDir(), "whep-check.sdp")
	if err := os.WriteFile(sdpPath, []byte(b.String()), 0o644); err != nil {
		return nil, err
	}
	logf("Playback SDP written to %s", sdpPath)
	args := []string{"-hide_banner", "-loglevel", "warning", "-protocol_whitelist", "file,udp,rtp",
		"-fflags", "nobuffer", "-flags", "low_delay", "-window_title", "whep-check", "-i", sdpPath}
	if o.player == "" {
		logf("Open it with: ffplay %s   (or VLC: vlc %s)", strings.Join(args, " "), sdpPath)
		return f, nil
	}
	f.cmd = exec.CommandContext(ctx, o.player, args...)
	f.cmd.Stdout, f.cmd.Stderr = os.Stdout, os.Stderr
	if err := f.cmd.Start(); err != nil {
		logf("WARNING: cannot start %s: %v", o.player, err)
		logf("Open it manually: ffplay %s", strings.Join(args, " "))
		f.cmd = nil
		return f, nil
	}
	logf("Started %s (pid %d)", o.player, f.cmd.Process.Pid)
	return f, nil
}

func (f *forwarder) write(video bool, pkt *rtp.Packet) {
	conn := f.audio
	if video {
		conn = f.video
	}
	if conn == nil {
		return
	}
	if buf, err := pkt.Marshal(); err == nil {
		_, _ = conn.Write(buf) // the player may not be listening yet
	}
}

func (f *forwarder) close() {
	for _, c := range []net.Conn{f.video, f.audio} {
		if c != nil {
			c.Close()
		}
	}
	if f.cmd != nil && f.cmd.Process != nil {
		_ = f.cmd.Process.Kill()
		_ = f.cmd.Wait()
	}
}

// recorder saves received media to files.
type recorder struct {
	mu    sync.Mutex
	video *h264writer.H264Writer
	audio *oggwriter.OggWriter
	dir   string
}

func newRecorder(dir string) (*recorder, error) {
	if err := os.MkdirAll(dir, 0o755); err != nil {
		return nil, fmt.Errorf("create record directory: %w", err)
	}
	return &recorder{dir: dir}, nil
}

func (r *recorder) write(video bool, mime string, pkt *rtp.Packet) {
	r.mu.Lock()
	defer r.mu.Unlock()
	var err error
	switch {
	case video && strings.EqualFold(mime, webrtc.MimeTypeH264):
		if r.video == nil {
			p := filepath.Join(r.dir, "video.h264")
			if r.video, err = h264writer.New(p); err != nil {
				logf("ERROR: %v", err)
				return
			}
			logf("Recording video to %s", p)
		}
		err = r.video.WriteRTP(pkt)
	case !video && strings.EqualFold(mime, webrtc.MimeTypeOpus):
		if r.audio == nil {
			p := filepath.Join(r.dir, "audio.ogg")
			if r.audio, err = oggwriter.New(p, 48000, 2); err != nil {
				logf("ERROR: %v", err)
				return
			}
			logf("Recording audio to %s", p)
		}
		err = r.audio.WriteRTP(pkt)
	default:
		return
	}
	if err != nil {
		logf("record write error: %v", err)
	}
}

func (r *recorder) close() {
	r.mu.Lock()
	defer r.mu.Unlock()
	if r.video != nil {
		r.video.Close()
	}
	if r.audio != nil {
		r.audio.Close()
	}
}
