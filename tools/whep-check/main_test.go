package main

import (
	"bytes"
	"context"
	"io"
	"net/http"
	"net/http/httptest"
	"os"
	"path/filepath"
	"runtime"
	"strings"
	"sync"
	"sync/atomic"
	"testing"
	"time"

	"github.com/pion/rtp"
	"github.com/pion/webrtc/v4"
)

// testServer is an in-process ice-lite WHEP endpoint that sends synthetic media.
type testServer struct {
	t         *testing.T
	status    int  // non-zero: reply to POST with this status and no answer
	breakICE  bool // close the server peer right after answering
	sendMedia bool
	media     func(pc *webrtc.PeerConnection, video, audio *webrtc.TrackLocalStaticRTP) // overrides sendSyntheticMedia
	deletes   atomic.Int32

	mu  sync.Mutex
	pcs []*webrtc.PeerConnection
}

func (s *testServer) ServeHTTP(w http.ResponseWriter, r *http.Request) {
	switch r.Method {
	case http.MethodDelete:
		s.deletes.Add(1)
		w.WriteHeader(http.StatusOK)
	case http.MethodPost:
		if s.status != 0 {
			http.Error(w, "test status", s.status)
			return
		}
		offer, _ := io.ReadAll(r.Body)
		answer, err := s.answer(string(offer))
		if err != nil {
			s.t.Errorf("test server answer: %v", err)
			http.Error(w, err.Error(), http.StatusInternalServerError)
			return
		}
		w.Header().Set("Content-Type", "application/sdp")
		w.Header().Set("Location", r.URL.Path+"?whepsign=test")
		w.WriteHeader(http.StatusCreated)
		_, _ = io.WriteString(w, answer)
	default:
		w.WriteHeader(http.StatusMethodNotAllowed)
	}
}

func (s *testServer) answer(offer string) (string, error) {
	se := webrtc.SettingEngine{}
	se.SetLite(true)
	se.SetIncludeLoopbackCandidate(true)
	se.SetNetworkTypes([]webrtc.NetworkType{webrtc.NetworkTypeUDP4})
	m := &webrtc.MediaEngine{}
	if err := m.RegisterDefaultCodecs(); err != nil {
		return "", err
	}
	pc, err := webrtc.NewAPI(webrtc.WithMediaEngine(m), webrtc.WithSettingEngine(se)).NewPeerConnection(webrtc.Configuration{})
	if err != nil {
		return "", err
	}
	s.mu.Lock()
	s.pcs = append(s.pcs, pc)
	s.mu.Unlock()

	video, err := webrtc.NewTrackLocalStaticRTP(webrtc.RTPCodecCapability{
		MimeType: webrtc.MimeTypeH264, ClockRate: 90000,
		SDPFmtpLine: "level-asymmetry-allowed=1;packetization-mode=1;profile-level-id=42e01f",
	}, "video", "test")
	if err != nil {
		return "", err
	}
	audio, err := webrtc.NewTrackLocalStaticRTP(webrtc.RTPCodecCapability{
		MimeType: webrtc.MimeTypeOpus, ClockRate: 48000, Channels: 2,
	}, "audio", "test")
	if err != nil {
		return "", err
	}
	for _, tr := range []webrtc.TrackLocal{video, audio} {
		if _, err := pc.AddTrack(tr); err != nil {
			return "", err
		}
	}
	if s.sendMedia {
		send := s.media
		if send == nil {
			send = sendSyntheticMedia
		}
		pc.OnConnectionStateChange(func(st webrtc.PeerConnectionState) {
			if st == webrtc.PeerConnectionStateConnected {
				go send(pc, video, audio)
			}
		})
	}

	if err := pc.SetRemoteDescription(webrtc.SessionDescription{Type: webrtc.SDPTypeOffer, SDP: offer}); err != nil {
		return "", err
	}
	ans, err := pc.CreateAnswer(nil)
	if err != nil {
		return "", err
	}
	gathered := webrtc.GatheringCompletePromise(pc)
	if err := pc.SetLocalDescription(ans); err != nil {
		return "", err
	}
	<-gathered
	sdp := pc.LocalDescription().SDP
	if s.breakICE {
		pc.Close()
	}
	return sdp, nil
}

func (s *testServer) close() {
	s.mu.Lock()
	defer s.mu.Unlock()
	for _, pc := range s.pcs {
		pc.Close()
	}
}

// bitWriter builds synthetic H.264 parameter sets.
type bitWriter struct {
	b    []byte
	nbit int
}

func (w *bitWriter) u(n int, v uint64) {
	for i := n - 1; i >= 0; i-- {
		if w.nbit%8 == 0 {
			w.b = append(w.b, 0)
		}
		if v>>uint(i)&1 == 1 {
			w.b[len(w.b)-1] |= 0x80 >> uint(w.nbit%8)
		}
		w.nbit++
	}
}

func (w *bitWriter) ue(v uint64) {
	v++
	n := 0
	for x := v; x > 1; x >>= 1 {
		n++
	}
	w.u(n, 0)
	w.u(n+1, v)
}

// nal adds RBSP trailing bits and emulation prevention, then the NAL header.
func (w *bitWriter) nal(header byte) []byte {
	w.u(1, 1)
	for w.nbit%8 != 0 {
		w.u(1, 0)
	}
	out := []byte{header}
	zeros := 0
	for _, b := range w.b {
		if zeros >= 2 && b <= 3 {
			out = append(out, 3)
			zeros = 0
		}
		out = append(out, b)
		if b == 0 {
			zeros++
		} else {
			zeros = 0
		}
	}
	return out
}

type testSPS struct {
	profile, constraints, level uint8
	mbW, mbH                    uint64
	cropBottom                  uint64 // in chroma units (2 lines for 4:2:0 progressive)
	vui                         bool
	timeScale, numUnits         uint64
}

func buildSPS(c testSPS) []byte {
	w := &bitWriter{}
	w.u(8, uint64(c.profile))
	w.u(8, uint64(c.constraints))
	w.u(8, uint64(c.level))
	w.ue(0) // sps id
	if c.profile == 100 {
		w.ue(1) // chroma_format_idc 4:2:0
		w.ue(0) // bit depth luma - 8
		w.ue(0) // bit depth chroma - 8
		w.u(1, 0)
		w.u(1, 0) // no scaling matrix
	}
	w.ue(0) // log2_max_frame_num_minus4
	w.ue(2) // poc type 2
	w.ue(1) // max ref frames
	w.u(1, 0)
	w.ue(c.mbW - 1)
	w.ue(c.mbH - 1)
	w.u(1, 1) // frame_mbs_only
	w.u(1, 1) // direct_8x8
	if c.cropBottom > 0 {
		w.u(1, 1)
		w.ue(0)
		w.ue(0)
		w.ue(0)
		w.ue(c.cropBottom)
	} else {
		w.u(1, 0)
	}
	w.u(1, b2u(c.vui))
	if c.vui {
		w.u(1, 1) // aspect ratio present
		w.u(8, 1) // 1:1
		w.u(1, 0) // overscan
		w.u(1, 1) // video signal type
		w.u(3, 5)
		w.u(1, 0) // limited range
		w.u(1, 1) // colour description
		w.u(8, 1)
		w.u(8, 1)
		w.u(8, 1)
		w.u(1, 0) // chroma loc
		w.u(1, 1) // timing
		w.u(32, c.numUnits)
		w.u(32, c.timeScale)
		w.u(1, 1) // fixed
		w.u(1, 0) // nal hrd
		w.u(1, 0) // vcl hrd
		w.u(1, 0) // pic struct
		w.u(1, 1) // bitstream restriction
		w.u(1, 1)
		w.ue(2)
		w.ue(1)
		w.ue(16)
		w.ue(16)
		w.ue(0) // max_num_reorder_frames
		w.ue(1) // max_dec_frame_buffering
	}
	return w.nal(0x67)
}

func buildPPS(cabac bool) []byte {
	w := &bitWriter{}
	w.ue(0)
	w.ue(0)
	w.u(1, b2u(cabac))
	w.u(1, 0)
	w.ue(0)
	return w.nal(0x68)
}

func b2u(b bool) uint64 {
	if b {
		return 1
	}
	return 0
}

var testSPS720 = buildSPS(testSPS{profile: 66, constraints: 0xc0, level: 31, mbW: 80, mbH: 45})

// stapA packs NAL units into one STAP-A payload.
func stapA(nals ...[]byte) []byte {
	out := []byte{0x78}
	for _, n := range nals {
		out = append(out, byte(len(n)>>8), byte(len(n)))
		out = append(out, n...)
	}
	return out
}

// sendSyntheticMedia writes RTP with recognizable H.264 NAL types; the slices are not decodable video.
func sendSyntheticMedia(pc *webrtc.PeerConnection, video, audio *webrtc.TrackLocalStaticRTP) {
	var seq uint16
	tick := time.NewTicker(20 * time.Millisecond)
	defer tick.Stop()
	pps := buildPPS(false)
	for i := 0; ; i++ {
		if pc.ConnectionState() != webrtc.PeerConnectionStateConnected {
			return
		}
		var payloads [][]byte
		if i%25 == 0 {
			payloads = [][]byte{stapA(testSPS720, pps), {0x65, 0x88, 0x84, 0x00}} // SPS+PPS, IDR
		} else {
			payloads = [][]byte{{0x41, 0x9a, 0x02, 0x00}} // non-IDR slice
		}
		for n, p := range payloads {
			seq++
			_ = video.WriteRTP(&rtp.Packet{Header: rtp.Header{
				Version: 2, SequenceNumber: seq, Timestamp: uint32(i * 1800), Marker: n == len(payloads)-1,
			}, Payload: p})
		}
		_ = audio.WriteRTP(&rtp.Packet{Header: rtp.Header{
			Version: 2, SequenceNumber: uint16(i), Timestamp: uint32(i * 960),
		}, Payload: []byte{0xfc, 0xff, 0xfe}})
		<-tick.C
	}
}

func startTestServer(t *testing.T, s *testServer) string {
	t.Helper()
	s.t = t
	srv := httptest.NewServer(s)
	t.Cleanup(func() {
		srv.Close()
		s.close()
	})
	return srv.URL + "/live/stream/whep.stream"
}

// captureLog collects log output for the duration of a test.
func captureLog(t *testing.T) *bytes.Buffer {
	t.Helper()
	var buf bytes.Buffer
	logMu.Lock()
	prev := logOut
	logOut = io.MultiWriter(prev, &buf)
	logMu.Unlock()
	t.Cleanup(func() {
		logMu.Lock()
		logOut = prev
		logMu.Unlock()
	})
	return &buf
}

func logText(buf *bytes.Buffer) string {
	logMu.Lock()
	defer logMu.Unlock()
	return buf.String()
}

// mkvBlock is a SimpleBlock found by walkMKV.
type mkvBlock struct {
	track uint64
	ts    int64
	key   bool
	data  []byte
}

// walkMKV parses the live Matroska written by whep-check.
func walkMKV(t *testing.T, b []byte) (codecs []string, blocks []mkvBlock) {
	t.Helper()
	readVint := func(keepMarker bool) (uint64, int) {
		if len(b) == 0 {
			return 0, 0
		}
		l := 1
		for l <= 8 && b[0]&(0x80>>uint(l-1)) == 0 {
			l++
		}
		if l > 8 || len(b) < l {
			return 0, 0
		}
		v := uint64(b[0])
		if !keepMarker {
			v &= uint64(0xff >> uint(l))
		}
		for i := 1; i < l; i++ {
			v = v<<8 | uint64(b[i])
		}
		return v, l
	}
	var clusterTs int64
	for len(b) > 0 {
		id, n := readVint(true)
		if n == 0 {
			t.Fatalf("bad element ID")
		}
		b = b[n:]
		size, m := readVint(false)
		if m == 0 {
			t.Fatalf("bad element size")
		}
		unknown := size == 1<<(7*uint(m))-1
		b = b[m:]
		if unknown {
			continue // live Segment/Cluster: children follow
		}
		if uint64(len(b)) < size {
			t.Fatalf("element %x truncated", id)
		}
		body := b[:size]
		b = b[size:]
		switch id {
		case idTracks, idTrackEntry:
			b = append(append([]byte(nil), body...), b...) // descend
		case idCodecID:
			codecs = append(codecs, string(body))
		case idTimestamp:
			clusterTs = 0
			for _, x := range body {
				clusterTs = clusterTs<<8 | int64(x)
			}
		case idSimpleBlock:
			tr := uint64(body[0] & 0x7f)
			rel := int16(uint16(body[1])<<8 | uint16(body[2]))
			blocks = append(blocks, mkvBlock{track: tr, ts: clusterTs + int64(rel), key: body[3]&0x80 != 0, data: body[4:]})
		}
	}
	return codecs, blocks
}

func TestCheckReceivesMediaAndDeletesResource(t *testing.T) {
	logs := captureLog(t)
	s := &testServer{sendMedia: true}
	url := startTestServer(t, s)
	dir := t.TempDir()
	code := check(context.Background(), options{
		url: url, duration: 2 * time.Second, iceTimeout: 10 * time.Second, interval: time.Second,
		buffer: 100 * time.Millisecond, recordDir: dir, includeLoopback: true,
	})
	if code != exitOK {
		t.Fatalf("exit code = %d, want %d", code, exitOK)
	}
	if got := s.deletes.Load(); got != 1 {
		t.Errorf("DELETE requests = %d, want 1", got)
	}
	files, _ := filepath.Glob(filepath.Join(dir, "whep-check-*.mkv"))
	if len(files) != 1 {
		t.Fatalf("recordings = %v, want one .mkv", files)
	}
	data, err := os.ReadFile(files[0])
	if err != nil {
		t.Fatal(err)
	}
	codecs, blocks := walkMKV(t, data)
	if strings.Join(codecs, ",") != "V_MPEG4/ISO/AVC,A_OPUS" {
		t.Errorf("codecs = %v", codecs)
	}
	var video, audio int
	for _, bl := range blocks {
		switch bl.track {
		case 1:
			if video == 0 && !bl.key {
				t.Errorf("first video block is not a keyframe")
			}
			video++
		case 2:
			audio++
		}
	}
	if video < 10 || audio < 10 {
		t.Errorf("blocks: video %d, audio %d; want both >= 10", video, audio)
	}
	out := logText(logs)
	for _, want := range []string{
		"Video format: H.264 Constrained Baseline, Level 3.1, 1280x720",
		"Audio format: Opus 48000 Hz, 2 channel(s) negotiated; stream: CELT mode, fullband",
		"Stats video H264:",
		"Final video H264:",
		"Media output: stream started with video H.264 1280x720, audio Opus 2 ch",
		"NOTE: SDP profile-level-id 42e01f (Constrained Baseline 3.1) differs from the stream SPS (Constrained Baseline 3.1)",
	} {
		if strings.HasPrefix(want, "NOTE:") {
			if strings.Contains(out, want) {
				t.Errorf("unexpected log line: %q", want)
			}
			continue
		}
		if !strings.Contains(out, want) {
			t.Errorf("log lacks %q", want)
		}
	}
}

func TestCheckReportsSignalingFailure(t *testing.T) {
	url := startTestServer(t, &testServer{status: http.StatusNotFound})
	code := check(context.Background(), options{url: url, iceTimeout: 5 * time.Second, interval: time.Second, includeLoopback: true})
	if code != exitSignaling {
		t.Fatalf("exit code = %d, want %d", code, exitSignaling)
	}
}

func TestCheckReportsUnreachableCandidate(t *testing.T) {
	s := &testServer{breakICE: true}
	url := startTestServer(t, s)
	code := check(context.Background(), options{url: url, iceTimeout: 3 * time.Second, interval: time.Second, includeLoopback: true})
	if code != exitICE {
		t.Fatalf("exit code = %d, want %d", code, exitICE)
	}
	if got := s.deletes.Load(); got != 1 {
		t.Errorf("DELETE requests = %d, want 1", got)
	}
}

func TestPlayPipesStreamToPlayer(t *testing.T) {
	if runtime.GOOS == "windows" {
		t.Skip("uses a shell script as the player")
	}
	dir := t.TempDir()
	got := filepath.Join(dir, "player-input.mkv")
	player := filepath.Join(dir, "player.sh")
	// The fake player reads the stream for a while and then exits, like a closed window.
	script := "#!/bin/sh\nhead -c 3000 > '" + got + "'\n"
	if err := os.WriteFile(player, []byte(script), 0o755); err != nil {
		t.Fatal(err)
	}
	logs := captureLog(t)
	s := &testServer{sendMedia: true}
	url := startTestServer(t, s)
	t0 := time.Now()
	code := check(context.Background(), options{
		url: url, duration: 0, iceTimeout: 10 * time.Second, interval: time.Second,
		buffer: 50 * time.Millisecond, play: true, player: player, includeLoopback: true,
	})
	if code != exitOK {
		t.Fatalf("exit code = %d, want %d", code, exitOK)
	}
	if time.Since(t0) > 30*time.Second {
		t.Errorf("check did not stop when the player exited")
	}
	if !strings.Contains(logText(logs), "Player closed") {
		t.Errorf("log lacks player exit")
	}
	data, err := os.ReadFile(got)
	if err != nil || len(data) < 4 || !bytes.Equal(data[:4], []byte{0x1a, 0x45, 0xdf, 0xa3}) {
		t.Fatalf("player input does not start with an EBML header (err %v, %d bytes)", err, len(data))
	}
}

func TestParseSPS(t *testing.T) {
	hd := buildSPS(testSPS{profile: 100, level: 42, mbW: 120, mbH: 68, cropBottom: 4, vui: true, timeScale: 100, numUnits: 1})
	s, err := parseSPS(hd)
	if err != nil {
		t.Fatal(err)
	}
	if s.Width != 1920 || s.Height != 1080 || !s.Cropped {
		t.Errorf("size = %dx%d cropped=%v, want 1920x1080 cropped", s.Width, s.Height, s.Cropped)
	}
	if s.FPS() != 50 || !s.FixedRate || !s.HasReorder || s.MaxReorder != 0 {
		t.Errorf("VUI fps=%v fixed=%v reorder=%v/%d", s.FPS(), s.FixedRate, s.HasReorder, s.MaxReorder)
	}
	lines := s.describe("CABAC")
	if want := "H.264 High, Level 4.2, 1920x1080 (coded 1920x1088), 4:2:0 8-bit, progressive"; !strings.HasPrefix(lines[0], want) {
		t.Errorf("describe = %q, want prefix %q", lines[0], want)
	}
	if !strings.Contains(lines[1], "50.00 fps (fixed), SAR 1:1, limited range, colour BT.709/BT.709/BT.709") {
		t.Errorf("VUI line = %q", lines[1])
	}

	s, err = parseSPS(testSPS720)
	if err != nil {
		t.Fatal(err)
	}
	if s.Width != 1280 || s.Height != 720 || s.VUI || h264ProfileName(s.Profile, s.Constraints) != "Constrained Baseline" {
		t.Errorf("720p SPS parsed as %+v", s)
	}
	if _, err := parseSPS([]byte{0x67, 0x42, 0xe0}); err == nil {
		t.Errorf("truncated SPS parsed without error")
	}
	if e, err := parsePPSEntropy(buildPPS(true)); err != nil || e != "CABAC" {
		t.Errorf("PPS entropy = %q, %v", e, err)
	}
}

func TestLevelCheck(t *testing.T) {
	s, err := parseSPS(buildSPS(testSPS{profile: 66, constraints: 0xc0, level: 31, mbW: 120, mbH: 68, cropBottom: 4}))
	if err != nil {
		t.Fatal(err)
	}
	w := levelCheck(s, 50)
	if !strings.Contains(w, "exceeds H.264 Level 3.1") || !strings.Contains(w, "Level 4.2 or higher") {
		t.Errorf("levelCheck = %q", w)
	}
	s.Level = 42
	if w := levelCheck(s, 50); w != "" {
		t.Errorf("1080p50 at Level 4.2 flagged: %q", w)
	}
	for plid, want := range map[string]string{"42001f": "Baseline 3.1", "42e01f": "Constrained Baseline 3.1", "640028": "High 4"} {
		if got := describeProfileLevelID(plid); got != want {
			t.Errorf("describeProfileLevelID(%s) = %q, want %q", plid, got, want)
		}
	}
}

func TestOpusTOC(t *testing.T) {
	toc, err := parseOpusTOC([]byte{0xfc, 0xff})
	if err != nil || toc.Mode != "CELT" || toc.Bandwidth != "FB" || toc.FrameDur != 20*time.Millisecond || !toc.Stereo || toc.Frames != 1 {
		t.Errorf("0xfc: %+v %v", toc, err)
	}
	toc, _ = parseOpusTOC([]byte{0x78})
	if toc.Mode != "Hybrid" || toc.Bandwidth != "FB" || toc.FrameDur != 20*time.Millisecond || toc.Stereo {
		t.Errorf("0x78: %+v", toc)
	}
	toc, _ = parseOpusTOC([]byte{0x59}) // SILK WB 60 ms, two frames
	if toc.Mode != "SILK" || toc.Bandwidth != "WB" || toc.Frames != 2 || toc.PacketDuration() != 120*time.Millisecond {
		t.Errorf("0x59: %+v", toc)
	}
}

func TestH264PayloadNALs(t *testing.T) {
	cases := []struct {
		name    string
		payload []byte
		want    []uint8
	}{
		{"IDR", []byte{0x65, 0x88}, []uint8{5}},
		{"STAP-A", stapA([]byte{0x67, 0x42}, []byte{0x68, 0xce}), []uint8{7, 8}},
		{"FU-A IDR start", []byte{0x7c, 0x85, 0x00}, []uint8{5}},
		{"FU-A IDR middle", []byte{0x7c, 0x05, 0x00}, nil},
		{"broken STAP-A", []byte{0x78, 0x00, 0x09, 0x67}, nil},
	}
	for _, c := range cases {
		var got []uint8
		for _, n := range h264PayloadNALs(c.payload) {
			got = append(got, n.typ)
		}
		if !bytes.Equal(got, c.want) {
			t.Errorf("%s: types %v, want %v", c.name, got, c.want)
		}
	}
}

func pkt(seq uint16, ts uint32, marker bool, payload []byte) *rtp.Packet {
	return &rtp.Packet{Header: rtp.Header{Version: 2, SequenceNumber: seq, Timestamp: ts, Marker: marker}, Payload: payload}
}

func TestTrackStatsLossLateDup(t *testing.T) {
	s := newTrackStats("audio", "audio/opus", 1, 48000, 2, "", nil)
	t0 := time.Now()
	// Wraps around 65535; 1 is lost, 3 arrives late, 4 is duplicated.
	for i, seq := range []uint16{65534, 65535, 0, 2, 4, 3, 4, 5} {
		s.onPacket(pkt(seq, uint32(seq)*960, false, []byte{0xfc}), t0.Add(time.Duration(i)*20*time.Millisecond))
	}
	c := s.snapshot()
	if c.lost != 1 || c.late != 1 || c.dup != 1 || c.packets != 8 {
		t.Errorf("lost %d late %d dup %d packets %d; want 1 1 1 8", c.lost, c.late, c.dup, c.packets)
	}
}

func TestTrackStatsKeyframesAndGOP(t *testing.T) {
	s := newTrackStats("video", "video/H264", 1, 90000, 0, "", nil)
	t0 := time.Now()
	seq := uint16(0)
	for f := 0; f < 100; f++ {
		ts := uint32(f * 1800)
		at := t0.Add(time.Duration(f) * 20 * time.Millisecond)
		if f%25 == 0 {
			// SPS/PPS, then an IDR in three FU-A fragments: one keyframe.
			for _, p := range [][]byte{stapA(testSPS720, buildPPS(false)), {0x7c, 0x85, 1}, {0x7c, 0x05, 2}, {0x7c, 0x45, 3}} {
				s.onPacket(pkt(seq, ts, p[0] == 0x7c && p[1]&0x40 != 0, p), at)
				seq++
			}
			continue
		}
		s.onPacket(pkt(seq, ts, true, []byte{0x41, 0x9a}), at)
		seq++
	}
	if s.keyframes != 4 || s.frames != 100 {
		t.Errorf("keyframes %d frames %d; want 4 and 100", s.keyframes, s.frames)
	}
	if g := s.gop.mean(); g < 0.499 || g > 0.501 {
		t.Errorf("GOP = %.3f s, want 0.5", g)
	}
	if s.format == nil || s.format.Width != 1280 {
		t.Errorf("format not parsed: %+v", s.format)
	}
}

func TestTrackStatsClassifiesGaps(t *testing.T) {
	run := func(rtpJump uint32) *trackStats {
		s := newTrackStats("video", "video/H264", 1, 90000, 0, "", nil)
		t0 := time.Now()
		at := t0
		ts := uint32(0)
		for f := 0; f < 60; f++ {
			if f == 30 {
				at = at.Add(800 * time.Millisecond) // nothing arrives for 800 ms
				ts += rtpJump
			}
			s.onPacket(pkt(uint16(f), ts, true, []byte{0x41, 0x9a}), at)
			at = at.Add(20 * time.Millisecond)
			ts += 1800
		}
		return s
	}
	delayed := run(0) // timestamps continue: the frames were held up in delivery
	if delayed.stalls != 1 || delayed.stallDelayed != 1 || !strings.Contains(delayed.verdict(), "delivery was delayed") {
		t.Errorf("delayed: stalls %d delayed %d verdict %q", delayed.stalls, delayed.stallDelayed, delayed.verdict())
	}
	paused := run(72000) // timestamps jump 800 ms: the source sent nothing
	if paused.stalls != 1 || paused.stallDelayed != 0 || !strings.Contains(paused.verdict(), "source sent no frames") {
		t.Errorf("paused: stalls %d delayed %d verdict %q", paused.stalls, paused.stallDelayed, paused.verdict())
	}
}

func TestJitterBufferReordersAndGivesUp(t *testing.T) {
	j := jitterBuffer{delay: 100 * time.Millisecond}
	var got []uint16
	var lost []int64
	emit := func(e jbEntry, l int64) {
		got = append(got, e.pkt.SequenceNumber)
		lost = append(lost, l)
	}
	t0 := time.Now()
	push := func(seq uint16, ms int) {
		j.push(pkt(seq, 0, false, nil), t0.Add(time.Duration(ms)*time.Millisecond), emit)
	}
	push(65535, 0)
	push(1, 10)  // 0 missing: waits
	push(0, 20)  // reorder resolved
	push(3, 30)  // 2 missing
	push(4, 40)  // still waiting
	push(5, 140) // 3 has waited 110 ms: 2 is given up
	push(2, 150) // too late
	want := []uint16{65535, 0, 1, 3, 4, 5}
	if len(got) != len(want) {
		t.Fatalf("released %v, want %v", got, want)
	}
	for i := range want {
		if got[i] != want[i] {
			t.Fatalf("released %v, want %v", got, want)
		}
	}
	if lost[3] != 1 || j.lost.Load() != 1 || j.late.Load() != 1 {
		t.Errorf("lost markers %v, lost %d, late %d", lost, j.lost.Load(), j.late.Load())
	}
}

func TestOutTrackSkipsToKeyframeAfterLoss(t *testing.T) {
	var buf bytes.Buffer
	var plis int
	o := newMediaOut(0, func(uint32) { plis++ })
	o.sinks = append(o.sinks, &sink{name: "test", w: &buf})
	tr := o.addTrack("video", "video/H264", 1, 90000, 0)
	t0 := time.Now()
	pps := buildPPS(false)
	seq := uint16(0)
	send := func(f int, payload []byte, marker, drop bool) {
		if !drop {
			tr.push(pkt(seq, uint32(f*1800), marker, payload), t0.Add(time.Duration(f)*20*time.Millisecond))
		}
		seq++
	}
	send(0, []byte{0x41, 0x01}, true, false) // P-frame before any keyframe: skipped
	send(1, stapA(testSPS720, pps), false, false)
	send(1, []byte{0x65, 0x88}, true, false) // keyframe: starts the stream
	send(2, []byte{0x41, 0x02}, true, false)
	send(3, []byte{0x7c, 0x81, 0x03}, false, false) // FU-A start
	send(3, []byte{0x7c, 0x01, 0x04}, false, true)  // lost middle fragment
	send(3, []byte{0x7c, 0x41, 0x05}, true, false)  // end: frame incomplete
	send(4, []byte{0x41, 0x06}, true, false)        // skipped: references the broken frame
	send(5, stapA(testSPS720, pps), false, false)
	send(5, []byte{0x65, 0x07}, true, false) // next keyframe resumes
	send(6, []byte{0x41, 0x08}, true, false)
	send(7, []byte{0x41, 0x09}, true, false) // pushes 6 out of the zero-delay buffer

	if tr.incomplete.Load() != 1 || tr.skipped.Load() != 2 || plis != 1 {
		t.Errorf("incomplete %d skipped %d PLI %d; want 1 2 1", tr.incomplete.Load(), tr.skipped.Load(), plis)
	}
	_, blocks := walkMKV(t, buf.Bytes())
	var keys []bool
	for _, b := range blocks {
		keys = append(keys, b.key)
	}
	// Written: frames 1 (key), 2, 5 (key), 6, 7.
	if len(blocks) != 5 || !keys[0] || keys[1] || !keys[2] {
		t.Errorf("blocks %d keys %v; want 5 blocks starting key, P, key", len(blocks), keys)
	}
	if len(blocks) > 1 && blocks[1].ts-blocks[0].ts != 20 {
		t.Errorf("frame spacing %d ms, want 20", blocks[1].ts-blocks[0].ts)
	}
}

func TestEBMLSize(t *testing.T) {
	for _, c := range []struct {
		n    uint64
		want []byte
	}{{0, []byte{0x80}}, {126, []byte{0xfe}}, {127, []byte{0x40, 0x7f}}, {16382, []byte{0x7f, 0xfe}}, {16383, []byte{0x20, 0x3f, 0xff}}} {
		if got := ebmlSize(c.n); !bytes.Equal(got, c.want) {
			t.Errorf("ebmlSize(%d) = %x, want %x", c.n, got, c.want)
		}
	}
}

func TestCandidateIP(t *testing.T) {
	ip := candidateIP("a=candidate:1 1 udp 2130706431 192.0.2.10 40000 typ host")
	if ip == nil || ip.String() != "192.0.2.10" {
		t.Fatalf("candidateIP = %v, want 192.0.2.10", ip)
	}
	if ip := candidateIP("a=candidate:broken"); ip != nil {
		t.Fatalf("candidateIP(broken) = %v, want nil", ip)
	}
}
