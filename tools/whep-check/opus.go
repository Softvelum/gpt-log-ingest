package main

import (
	"errors"
	"fmt"
	"time"
)

// opusTOC describes the table-of-contents byte of an Opus packet (RFC 6716, section 3.1).
type opusTOC struct {
	Mode      string // SILK, Hybrid or CELT
	Bandwidth string // NB, MB, WB, SWB or FB
	FrameDur  time.Duration
	Stereo    bool
	Frames    int // frames in the packet
}

// PacketDuration is the audio duration carried by the packet.
func (t opusTOC) PacketDuration() time.Duration { return t.FrameDur * time.Duration(t.Frames) }

func parseOpusTOC(p []byte) (opusTOC, error) {
	if len(p) < 1 {
		return opusTOC{}, errors.New("empty Opus packet")
	}
	cfg := int(p[0] >> 3)
	t := opusTOC{Stereo: p[0]&0x04 != 0}
	ms := func(v float64) time.Duration { return time.Duration(v * float64(time.Millisecond)) }
	switch {
	case cfg < 12:
		t.Mode = "SILK"
		t.Bandwidth = []string{"NB", "MB", "WB"}[cfg/4]
		t.FrameDur = ms([]float64{10, 20, 40, 60}[cfg%4])
	case cfg < 16:
		t.Mode = "Hybrid"
		t.Bandwidth = []string{"SWB", "FB"}[(cfg-12)/2]
		t.FrameDur = ms([]float64{10, 20}[cfg%2])
	default:
		t.Mode = "CELT"
		t.Bandwidth = []string{"NB", "WB", "SWB", "FB"}[(cfg-16)/4]
		t.FrameDur = ms([]float64{2.5, 5, 10, 20}[cfg%4])
	}
	switch p[0] & 0x03 {
	case 0:
		t.Frames = 1
	case 1, 2:
		t.Frames = 2
	case 3:
		if len(p) < 2 {
			return t, errors.New("truncated Opus code 3 packet")
		}
		t.Frames = int(p[1] & 0x3f)
	}
	return t, nil
}

func (t opusTOC) String() string {
	ch := "mono"
	if t.Stereo {
		ch = "stereo"
	}
	bw := map[string]string{"NB": "narrowband 4 kHz", "MB": "mediumband 6 kHz", "WB": "wideband 8 kHz",
		"SWB": "super-wideband 12 kHz", "FB": "fullband 20 kHz"}[t.Bandwidth]
	return fmt.Sprintf("%s mode, %s, %s, %v frames x%d (%v per packet)",
		t.Mode, bw, ch, t.FrameDur, t.Frames, t.PacketDuration())
}
