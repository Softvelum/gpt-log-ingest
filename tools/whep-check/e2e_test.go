package main

import (
	"context"
	"errors"
	"io"
	"os"
	"os/exec"
	"path/filepath"
	"runtime"
	"strings"
	"testing"
	"time"

	"github.com/pion/rtp"
	"github.com/pion/rtp/codecs"
	"github.com/pion/webrtc/v4"
	"github.com/pion/webrtc/v4/pkg/media/h264reader"
	"github.com/pion/webrtc/v4/pkg/media/oggreader"
)

// TestEndToEndWithRealMedia sends real H.264 and Opus (made by ffmpeg) with a
// resolution switch, reordering, one unrecoverable loss and a delivery stall,
// then checks the statistics and that ffmpeg decodes the recording and the
// live player stream without errors.
// It is skipped when ffmpeg is not installed.
func TestEndToEndWithRealMedia(t *testing.T) {
	ffmpeg, err := exec.LookPath("ffmpeg")
	if err != nil {
		t.Skip("ffmpeg not found in PATH")
	}
	if runtime.GOOS == "windows" {
		t.Skip("uses a shell script as the player")
	}
	dir := t.TempDir()
	vfile, afile := filepath.Join(dir, "v.h264"), filepath.Join(dir, "a.ogg")
	low := filepath.Join(dir, "low.h264")
	for _, args := range [][]string{
		// A short low-resolution start, as when WebRTC ABR begins on a lower rendition.
		{"-f", "lavfi", "-i", "testsrc2=size=320x240:rate=50", "-t", "0.4", "-c:v", "libx264", "-profile:v", "baseline",
			"-level", "2.2", "-bf", "0", "-g", "25", "-pix_fmt", "yuv420p", "-f", "h264", low},
		{"-f", "lavfi", "-i", "testsrc2=size=1280x720:rate=50", "-t", "5", "-c:v", "libx264", "-profile:v", "baseline",
			"-level", "3.2", "-bf", "0", "-g", "25", "-pix_fmt", "yuv420p", "-f", "h264", vfile},
		{"-f", "lavfi", "-i", "sine=frequency=440:sample_rate=48000", "-t", "5", "-ac", "2", "-c:a", "libopus",
			"-b:a", "96k", "-page_duration", "20000", afile},
	} {
		cmd := exec.Command(ffmpeg, append([]string{"-hide_banner", "-loglevel", "error", "-y"}, args...)...)
		if out, err := cmd.CombinedOutput(); err != nil {
			t.Fatalf("ffmpeg: %v\n%s", err, out)
		}
	}
	frames := append(readAccessUnits(t, low), readAccessUnits(t, vfile)...)
	opus := readOpusPackets(t, afile)

	logs := captureLog(t)
	s := &testServer{sendMedia: true, media: func(pc *webrtc.PeerConnection, video, audio *webrtc.TrackLocalStaticRTP) {
		sendImpairedMedia(pc, video, audio, frames, opus)
	}}
	url := startTestServer(t, s)
	rec := filepath.Join(dir, "rec")
	// The "player" decodes the live stream from its standard input with ffmpeg.
	playerErrs := filepath.Join(dir, "player-errors.txt")
	player := filepath.Join(dir, "player.sh")
	// It uses the same input options as ffplay gets from -play.
	script := "#!/bin/sh\nexec '" + ffmpeg + "' -hide_banner -v error " + strings.Join(ffInputArgs, " ") +
		" -f null - 2> '" + playerErrs + "'\n"
	if err := os.WriteFile(player, []byte(script), 0o755); err != nil {
		t.Fatal(err)
	}
	code := check(context.Background(), options{
		url: url, duration: 6 * time.Second, iceTimeout: 10 * time.Second, interval: 2 * time.Second,
		buffer: 200 * time.Millisecond, recordDir: rec, play: true, player: player, includeLoopback: true,
	})
	if code != exitOK {
		t.Fatalf("exit code = %d", code)
	}
	out := logText(logs)
	for _, want := range []string{
		"Video format: H.264 Constrained Baseline, Level 2.2, 320x240",
		"Video format changed (320x240 -> 1280x720): H.264 Constrained Baseline, Level 3.2, 1280x720",
		"Audio format: Opus 48000 Hz, 2 channel(s) negotiated; stream: CELT mode, fullband",
		"frames were delayed in delivery",
		"lost 1 (",
		"TIMING: frames arrived in bursts after gaps while their RTP timestamps continued",
	} {
		if !strings.Contains(out, want) {
			t.Errorf("log lacks %q", want)
		}
	}
	for _, l := range strings.Split(out, "\n") {
		if strings.Contains(l, "Final video") && strings.Contains(l, "NACKed 0,") {
			t.Errorf("the NACK for the lost packet was not counted: %s", l)
		}
	}

	files, _ := filepath.Glob(filepath.Join(rec, "*.mkv"))
	if len(files) != 1 {
		t.Fatalf("recordings: %v", files)
	}
	dec := exec.Command(ffmpeg, "-hide_banner", "-v", "error", "-i", files[0], "-f", "null", "-")
	msg, err := dec.CombinedOutput()
	if err != nil || len(strings.TrimSpace(string(msg))) > 0 {
		t.Errorf("decoding the recording reported errors (%v):\n%s", err, msg)
	}
	if msg, err := os.ReadFile(playerErrs); err != nil || len(strings.TrimSpace(string(msg))) > 0 {
		t.Errorf("decoding the live player stream reported errors (%v):\n%s", err, msg)
	}
	probe := exec.Command(ffmpeg, "-hide_banner", "-i", files[0])
	info, _ := probe.CombinedOutput()
	// The header carries the first keyframe's format (320x240); the switch to 720p is in-band.
	if !strings.Contains(string(info), "h264") || !strings.Contains(string(info), "opus") {
		t.Errorf("unexpected recording streams:\n%s", info)
	}
}

// readAccessUnits groups an Annex B stream into access units (one slice per frame).
func readAccessUnits(t *testing.T, path string) [][]byte {
	t.Helper()
	f, err := os.Open(path)
	if err != nil {
		t.Fatal(err)
	}
	defer f.Close()
	r, err := h264reader.NewReader(f)
	if err != nil {
		t.Fatal(err)
	}
	var aus [][]byte
	var cur []byte
	for {
		nal, err := r.NextNAL()
		if errors.Is(err, io.EOF) {
			break
		}
		if err != nil {
			t.Fatal(err)
		}
		cur = append(cur, 0, 0, 0, 1)
		cur = append(cur, nal.Data...)
		if nal.UnitType == h264reader.NalUnitTypeCodedSliceIdr || nal.UnitType == h264reader.NalUnitTypeCodedSliceNonIdr {
			aus = append(aus, cur)
			cur = nil
		}
	}
	return aus
}

func readOpusPackets(t *testing.T, path string) [][]byte {
	t.Helper()
	f, err := os.Open(path)
	if err != nil {
		t.Fatal(err)
	}
	defer f.Close()
	r, _, err := oggreader.NewWith(f)
	if err != nil {
		t.Fatal(err)
	}
	var pkts [][]byte
	for {
		p, _, err := r.ParseNextPage()
		if err != nil {
			break
		}
		if len(p) == 0 || strings.HasPrefix(string(p), "OpusTags") {
			continue
		}
		pkts = append(pkts, p)
	}
	return pkts
}

func sendImpairedMedia(pc *webrtc.PeerConnection, video, audio *webrtc.TrackLocalStaticRTP, frames, opus [][]byte) {
	vp := rtp.NewPacketizer(1200, 96, 1, &codecs.H264Payloader{}, rtp.NewRandomSequencer(), 90000)
	ap := rtp.NewPacketizer(1200, 111, 2, &codecs.OpusPayloader{}, rtp.NewRandomSequencer(), 48000)
	dropped := false
	next := time.Now()
	var behind time.Duration
	for i, au := range frames {
		if pc.ConnectionState() != webrtc.PeerConnectionStateConnected {
			return
		}
		pkts := vp.Packetize(au, 1800)
		switch {
		case i == 60 && len(pkts) >= 3: // reordering
			pkts[0], pkts[1] = pkts[1], pkts[0]
		case !dropped && i > 80 && i%25 != 0 && len(pkts) >= 3: // one loss, never retransmitted
			pkts = append(pkts[:1], pkts[2:]...)
			dropped = true
		}
		for _, p := range pkts {
			_ = video.WriteRTP(p)
		}
		if i < len(opus) {
			for _, p := range ap.Packetize(opus[i], 960) {
				_ = audio.WriteRTP(p)
			}
		}
		switch {
		case i == 140: // delivery stall: hold everything, then catch up in a fast burst
			time.Sleep(600 * time.Millisecond)
			next = time.Now()
			behind = 600 * time.Millisecond
		case behind > 0:
			next = next.Add(4 * time.Millisecond)
			behind -= 16 * time.Millisecond
		default:
			next = next.Add(20 * time.Millisecond)
		}
		if d := time.Until(next); d > 0 {
			time.Sleep(d)
		}
	}
}
