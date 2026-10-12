package main

import (
	"context"
	"io"
	"net/http"
	"net/http/httptest"
	"os"
	"path/filepath"
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
		pc.OnConnectionStateChange(func(st webrtc.PeerConnectionState) {
			if st == webrtc.PeerConnectionStateConnected {
				go sendSyntheticMedia(pc, video, audio)
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

// sendSyntheticMedia writes RTP with recognizable H.264 NAL types; it is not decodable video.
func sendSyntheticMedia(pc *webrtc.PeerConnection, video, audio *webrtc.TrackLocalStaticRTP) {
	var seq uint16
	tick := time.NewTicker(20 * time.Millisecond)
	defer tick.Stop()
	for i := 0; ; i++ {
		if pc.ConnectionState() != webrtc.PeerConnectionStateConnected {
			return
		}
		var nals [][]byte
		if i%25 == 0 {
			nals = [][]byte{{0x67, 0x42, 0xe0, 0x1f}, {0x68, 0xce, 0x38, 0x80}, {0x65, 0x88, 0x84, 0x00}} // SPS, PPS, IDR
		} else {
			nals = [][]byte{{0x41, 0x9a, 0x02, 0x00}} // non-IDR slice
		}
		for n, nal := range nals {
			seq++
			_ = video.WriteRTP(&rtp.Packet{Header: rtp.Header{
				Version: 2, SequenceNumber: seq, Timestamp: uint32(i * 1800), Marker: n == len(nals)-1,
			}, Payload: nal})
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

func TestCheckReceivesMediaAndDeletesResource(t *testing.T) {
	s := &testServer{sendMedia: true}
	url := startTestServer(t, s)
	dir := t.TempDir()
	code := check(context.Background(), options{
		url: url, duration: 2 * time.Second, iceTimeout: 10 * time.Second,
		recordDir: dir, includeLoopback: true,
	})
	if code != exitOK {
		t.Fatalf("exit code = %d, want %d", code, exitOK)
	}
	if got := s.deletes.Load(); got != 1 {
		t.Errorf("DELETE requests = %d, want 1", got)
	}
	for _, name := range []string{"video.h264", "audio.ogg"} {
		info, err := os.Stat(filepath.Join(dir, name))
		if err != nil || info.Size() == 0 {
			t.Errorf("recorded %s missing or empty: %v", name, err)
		}
	}
}

func TestCheckReportsSignalingFailure(t *testing.T) {
	url := startTestServer(t, &testServer{status: http.StatusNotFound})
	code := check(context.Background(), options{url: url, iceTimeout: 5 * time.Second, includeLoopback: true})
	if code != exitSignaling {
		t.Fatalf("exit code = %d, want %d", code, exitSignaling)
	}
}

func TestCheckReportsUnreachableCandidate(t *testing.T) {
	s := &testServer{breakICE: true}
	url := startTestServer(t, s)
	code := check(context.Background(), options{url: url, iceTimeout: 3 * time.Second, includeLoopback: true})
	if code != exitICE {
		t.Fatalf("exit code = %d, want %d", code, exitICE)
	}
	if got := s.deletes.Load(); got != 1 {
		t.Errorf("DELETE requests = %d, want 1", got)
	}
}

func TestH264Keyframe(t *testing.T) {
	cases := []struct {
		name    string
		payload []byte
		want    bool
	}{
		{"IDR", []byte{0x65, 0x88}, true},
		{"SPS", []byte{0x67, 0x42}, true},
		{"non-IDR", []byte{0x41, 0x9a}, false},
		{"STAP-A with SPS", []byte{0x78, 0x00, 0x02, 0x67, 0x42, 0x00, 0x02, 0x68, 0xce}, true},
		{"STAP-A without key", []byte{0x78, 0x00, 0x02, 0x41, 0x9a}, false},
		{"FU-A IDR start", []byte{0x7c, 0x85, 0x00}, true},
		{"FU-A IDR middle", []byte{0x7c, 0x05, 0x00}, false},
		{"short", []byte{0x65}, false},
	}
	for _, c := range cases {
		if got := h264Keyframe(c.payload); got != c.want {
			t.Errorf("%s: h264Keyframe = %v, want %v", c.name, got, c.want)
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
