// whep-check is a WHEP playback client for diagnosing WebRTC delivery.
//
// It performs the WHEP signaling exchange, reports the server answer
// (ICE mode, candidates, codecs), follows ICE/DTLS progress, and analyzes the
// received RTP media: format, loss, retransmissions, keyframes and frame
// timing. It can also play or record the stream as live Matroska.
package main

import (
	"bytes"
	"context"
	"flag"
	"fmt"
	"io"
	"net"
	"net/http"
	"net/url"
	"os"
	"os/signal"
	"path/filepath"
	"strings"
	"sync"
	"syscall"
	"time"

	"github.com/pion/interceptor"
	"github.com/pion/logging"
	"github.com/pion/rtcp"
	"github.com/pion/webrtc/v4"
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
	interval    time.Duration
	stun        string
	play        bool
	player      string
	buffer      time.Duration
	recordDir   string
	verbose     bool
	insecureTLS bool

	// includeLoopback lets the in-process tests connect over 127.0.0.1.
	includeLoopback bool
}

var (
	start            = time.Now()
	logOut io.Writer = os.Stdout
	logMu  sync.Mutex
)

func logf(format string, args ...any) {
	logMu.Lock()
	defer logMu.Unlock()
	fmt.Fprintf(logOut, "[%s +%6.2fs] %s\n", time.Now().UTC().Format("15:04:05.000"),
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
	flag.DurationVar(&o.interval, "interval", 5*time.Second, "statistics interval")
	flag.StringVar(&o.stun, "stun", "", "optional STUN server, e.g. stun:stun.l.google.com:19302 (not needed for ice-lite servers)")
	flag.BoolVar(&o.play, "play", false, "play the stream: live Matroska on the player's standard input")
	flag.StringVar(&o.player, "player", "ffplay", "player for -play (ffplay, mpv or vlc); empty = write the stream to stdout and logs to stderr")
	flag.DurationVar(&o.buffer, "buffer", 300*time.Millisecond, "jitter buffer for -play/-record: how long to wait for reordered or retransmitted packets")
	flag.StringVar(&o.recordDir, "record", "", "save the stream as <dir>/whep-check-<UTC time>.mkv")
	flag.BoolVar(&o.verbose, "v", false, "verbose ICE/DTLS logging from the WebRTC stack")
	flag.BoolVar(&o.insecureTLS, "k", false, "skip TLS certificate verification for the WHEP HTTP requests")
	flag.Usage = func() {
		fmt.Fprintf(os.Stderr, "Usage: %s [options] <whep-url>\n\nOptions:\n", filepath.Base(os.Args[0]))
		flag.PrintDefaults()
		fmt.Fprintf(os.Stderr, "\nExit codes: 0 media received, 1 usage, 2 signaling failed, 3 ICE/DTLS failed, 4 connected but no media\n")
	}
	flag.Parse()
	if flag.NArg() != 1 || o.interval <= 0 || o.buffer < 0 {
		flag.Usage()
		return exitUsage
	}
	o.url = flag.Arg(0)
	durationSet := false
	flag.Visit(func(f *flag.Flag) { durationSet = durationSet || f.Name == "duration" })
	if o.play && !durationSet {
		o.duration = 0 // watch until Ctrl+C or until the player is closed
	}
	if o.play && o.player == "" {
		logOut = os.Stderr // stdout carries the media stream
		signal.Ignore(syscall.SIGPIPE)
	}

	ctx, cancel := signal.NotifyContext(context.Background(), os.Interrupt, syscall.SIGTERM)
	defer cancel()

	return check(ctx, o)
}

func check(ctx context.Context, o options) int {
	counter := newRTCPCounter()
	pc, err := newPeerConnection(o, counter)
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

	pli := func(ssrc uint32) {
		_ = pc.WriteRTCP([]rtcp.Packet{&rtcp.PictureLossIndication{MediaSSRC: ssrc}})
	}

	// Optional media output (player and/or file).
	var out *mediaOut
	if o.play || o.recordDir != "" {
		out = newMediaOut(o.buffer, pli)
		defer out.close()
		if o.recordDir != "" {
			if err := out.addRecordFile(o.recordDir); err != nil {
				logf("ERROR: %v", err)
				return exitUsage
			}
		}
		if o.play {
			if o.player == "" {
				out.addStdout()
			} else if err := out.startPlayer(ctx, o.player); err != nil {
				logf("ERROR: cannot start %s: %v (install it, pass -player, or use -player \"\" to pipe the stream)", o.player, err)
				return exitUsage
			}
		}
	}
	var playerDone <-chan struct{}
	if out != nil && out.cmd != nil {
		playerDone = out.playerDone
	}

	var (
		statsMu sync.Mutex
		stats   []*trackStats
	)
	pc.OnTrack(func(track *webrtc.TrackRemote, _ *webrtc.RTPReceiver) {
		c := track.Codec()
		kind := track.Kind().String()
		st := newTrackStats(kind, c.MimeType, uint32(track.SSRC()), c.ClockRate, c.Channels, c.SDPFmtpLine, counter)
		statsMu.Lock()
		stats = append(stats, st)
		statsMu.Unlock()
		logf("Track: %s %s pt=%d clock=%d channels=%d ssrc=%d", kind, c.MimeType, c.PayloadType, c.ClockRate, c.Channels, st.ssrc)
		var fb []string
		for _, f := range c.RTCPFeedback {
			fb = append(fb, strings.TrimSpace(f.Type+" "+f.Parameter))
		}
		if len(fb) > 0 {
			logf("  RTCP feedback: %s", strings.Join(fb, ", "))
		}

		var ot *outTrack
		if out != nil {
			ot = out.addTrack(kind, c.MimeType, st.ssrc, c.ClockRate, c.Channels)
		}
		if track.Kind() == webrtc.RTPCodecTypeVideo {
			go requestFirstKeyframe(ctx, st, pli)
		}
		for {
			pkt, _, err := track.ReadRTP()
			if err != nil {
				if err != io.EOF {
					logf("%s track read ended: %v", kind, err)
				}
				return
			}
			now := time.Now()
			st.onPacket(pkt, now)
			if ot != nil {
				ot.push(pkt, now)
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
	if out != nil {
		out.expect(answerSends(answerSDP, "video"), answerSends(answerSDP, "audio"))
	}

	if err := pc.SetRemoteDescription(webrtc.SessionDescription{Type: webrtc.SDPTypeAnswer, SDP: answerSDP}); err != nil {
		logf("ERROR: set remote description: %v", err)
		return exitSignaling
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
	ticker := time.NewTicker(o.interval)
	defer ticker.Stop()
	snapshot := func() []*trackStats {
		statsMu.Lock()
		defer statsMu.Unlock()
		return append([]*trackStats(nil), stats...)
	}
loop:
	for {
		select {
		case now := <-ticker.C:
			all := snapshot()
			if len(all) == 0 {
				logf("Stats: no tracks yet")
			}
			for _, st := range all {
				logf("%s", st.intervalLine(now))
			}
			if out != nil {
				logf("%s", out.statsLine())
			}
		case reason := <-failed:
			logf("Connection lost: %s", reason)
			break loop
		case <-playerDone:
			logf("Player closed")
			break loop
		case <-deadline:
			break loop
		case <-ctx.Done():
			break loop
		}
	}

	all := snapshot()
	var total uint64
	for _, st := range all {
		for _, l := range st.finalLines() {
			logf("%s", l)
		}
		st.mu.Lock()
		total += st.packets
		st.mu.Unlock()
	}
	if out != nil {
		logf("%s", out.statsLine())
	}
	if total == 0 {
		logf("RESULT: connected, but no RTP media received")
		return exitNoMedia
	}
	logf("RESULT: OK, media received")
	return exitOK
}

// requestFirstKeyframe asks for a keyframe until the first one arrives, so
// the format is known quickly. It stops afterwards to keep the server's own
// keyframe interval measurable.
func requestFirstKeyframe(ctx context.Context, st *trackStats, pli func(uint32)) {
	for i := 0; i < 5; i++ {
		st.mu.Lock()
		have := st.haveKey
		st.mu.Unlock()
		if have {
			return
		}
		pli(st.ssrc)
		select {
		case <-ctx.Done():
			return
		case <-time.After(time.Second):
		}
	}
}

func newPeerConnection(o options, counter *rtcpCounter) (*webrtc.PeerConnection, error) {
	m := &webrtc.MediaEngine{}
	if err := m.RegisterDefaultCodecs(); err != nil {
		return nil, err
	}
	ir := &interceptor.Registry{}
	ir.Add(counter) // before the NACK generator, so its requests are counted
	if err := webrtc.RegisterDefaultInterceptors(m, ir); err != nil {
		return nil, err
	}
	se := webrtc.SettingEngine{}
	lf := logging.NewDefaultLoggerFactory()
	lf.Writer = logWriter{}
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

// logWriter sends Pion's own log output to the current log destination.
type logWriter struct{}

func (logWriter) Write(p []byte) (int, error) {
	logMu.Lock()
	defer logMu.Unlock()
	return logOut.Write(p)
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

// answerSends reports whether the SDP answer has an active m-line of this
// kind in which the server sends media.
func answerSends(sdp, kind string) bool {
	in, sends := false, false
	for _, l := range sdpLines(sdp) {
		if strings.HasPrefix(l, "m=") {
			if in && sends {
				return true
			}
			f := strings.Fields(strings.TrimPrefix(l, "m="))
			in = len(f) > 1 && f[0] == kind && f[1] != "0"
			sends = true // sendrecv is the default direction
			continue
		}
		if in && (l == "a=recvonly" || l == "a=inactive") {
			sends = false
		}
	}
	return in && sends
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
