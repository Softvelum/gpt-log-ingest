package main

import (
	"encoding/hex"
	"errors"
	"fmt"
	"strings"
)

// bitReader reads an H.264 RBSP bit by bit. The first error sticks.
type bitReader struct {
	b   []byte
	pos int // bit position
	err error
}

var errShortSPS = errors.New("truncated parameter set")

func (r *bitReader) u(n int) uint64 {
	var v uint64
	for i := 0; i < n; i++ {
		if r.err != nil {
			return 0
		}
		if r.pos >= len(r.b)*8 {
			r.err = errShortSPS
			return 0
		}
		bit := (r.b[r.pos/8] >> (7 - uint(r.pos%8))) & 1
		v = v<<1 | uint64(bit)
		r.pos++
	}
	return v
}

func (r *bitReader) flag() bool { return r.u(1) == 1 }

func (r *bitReader) ue() uint64 {
	zeros := 0
	for r.err == nil && r.u(1) == 0 {
		zeros++
		if zeros > 31 {
			r.err = errors.New("invalid Exp-Golomb code")
			return 0
		}
	}
	if zeros == 0 {
		return 0
	}
	return (1<<uint(zeros) - 1) + r.u(zeros)
}

func (r *bitReader) se() int64 {
	v := r.ue()
	if v%2 == 1 {
		return int64(v+1) / 2
	}
	return -int64(v / 2)
}

// rbsp removes emulation-prevention bytes (00 00 03 -> 00 00).
func rbsp(nal []byte) []byte {
	out := make([]byte, 0, len(nal))
	zeros := 0
	for _, b := range nal {
		if zeros >= 2 && b == 3 {
			zeros = 0
			continue
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

// spsInfo holds the H.264 sequence parameter set fields that whep-check reports.
type spsInfo struct {
	Profile, Constraints, Level uint8
	ChromaFormat                uint64
	BitDepthLuma, BitDepthCh    uint64
	FrameMbsOnly                bool
	MaxRefFrames                uint64
	POCType                     uint64
	MbWidth, MbHeight           uint64 // in macroblocks (frame height for interlaced content)
	Width, Height               int    // after cropping
	Cropped                     bool

	VUI            bool
	SARNum, SARDen uint64
	VideoFormat    int // -1 if absent
	FullRange      bool
	HasColour      bool
	Primaries      uint64
	Transfer       uint64
	Matrix         uint64
	HasTiming      bool
	NumUnits       uint64
	TimeScale      uint64
	FixedRate      bool
	HRDBitrate     uint64 // bits/s, NAL HRD first CPB; 0 if absent
	HRDCBR         bool
	HasReorder     bool
	MaxReorder     uint64
	MaxDecBuffer   uint64
}

// FPS returns the frame rate signalled in the VUI timing info, or 0.
func (s *spsInfo) FPS() float64 {
	if !s.HasTiming || s.NumUnits == 0 {
		return 0
	}
	return float64(s.TimeScale) / float64(2*s.NumUnits)
}

// parseSPS parses an SPS NAL unit including its one-byte NAL header.
func parseSPS(nal []byte) (*spsInfo, error) {
	if len(nal) < 4 || nal[0]&0x1f != 7 {
		return nil, errors.New("not an SPS NAL unit")
	}
	r := &bitReader{b: rbsp(nal[1:])}
	s := &spsInfo{VideoFormat: -1}
	s.Profile = uint8(r.u(8))
	s.Constraints = uint8(r.u(8))
	s.Level = uint8(r.u(8))
	r.ue() // seq_parameter_set_id
	s.ChromaFormat = 1
	switch s.Profile {
	case 100, 110, 122, 244, 44, 83, 86, 118, 128, 138, 139, 134, 135:
		s.ChromaFormat = r.ue()
		if s.ChromaFormat == 3 {
			r.u(1) // separate_colour_plane_flag
		}
		s.BitDepthLuma = r.ue()
		s.BitDepthCh = r.ue()
		r.u(1)        // qpprime_y_zero_transform_bypass_flag
		if r.flag() { // seq_scaling_matrix_present_flag
			n := 8
			if s.ChromaFormat == 3 {
				n = 12
			}
			for i := 0; i < n; i++ {
				if r.flag() {
					size := 16
					if i >= 6 {
						size = 64
					}
					last, next := int64(8), int64(8)
					for j := 0; j < size && r.err == nil; j++ {
						if next != 0 {
							next = (last + r.se() + 256) % 256
						}
						if next != 0 {
							last = next
						}
					}
				}
			}
		}
	}
	r.ue() // log2_max_frame_num_minus4
	s.POCType = r.ue()
	switch s.POCType {
	case 0:
		r.ue() // log2_max_pic_order_cnt_lsb_minus4
	case 1:
		r.u(1)
		r.se()
		r.se()
		n := r.ue()
		if n > 255 {
			return nil, errors.New("invalid num_ref_frames_in_pic_order_cnt_cycle")
		}
		for i := uint64(0); i < n; i++ {
			r.se()
		}
	}
	s.MaxRefFrames = r.ue()
	r.u(1) // gaps_in_frame_num_value_allowed_flag
	s.MbWidth = r.ue() + 1
	mapUnits := r.ue() + 1
	s.FrameMbsOnly = r.flag()
	if !s.FrameMbsOnly {
		r.u(1) // mb_adaptive_frame_field_flag
	}
	r.u(1) // direct_8x8_inference_flag
	frameMult := uint64(2)
	if s.FrameMbsOnly {
		frameMult = 1
	}
	s.MbHeight = mapUnits * frameMult
	var cl, cr, ct, cb uint64
	if r.flag() {
		cl, cr, ct, cb = r.ue(), r.ue(), r.ue(), r.ue()
		s.Cropped = true
	}
	if r.err != nil {
		return nil, r.err
	}
	cropX, cropY := uint64(1), frameMult
	if s.ChromaFormat == 1 || s.ChromaFormat == 2 {
		cropX = 2
	}
	if s.ChromaFormat == 1 {
		cropY = 2 * frameMult
	}
	w := s.MbWidth*16 - (cl+cr)*cropX
	h := s.MbHeight*16 - (ct+cb)*cropY
	if w > 1<<16 || h > 1<<16 {
		return nil, errors.New("invalid cropping")
	}
	s.Width, s.Height = int(w), int(h)

	s.VUI = r.flag()
	if s.VUI {
		parseVUI(r, s)
	}
	// A truncated VUI still leaves the core fields usable.
	if r.err != nil && !s.VUI {
		return nil, r.err
	}
	return s, nil
}

func parseVUI(r *bitReader, s *spsInfo) {
	if r.flag() { // aspect_ratio_info_present_flag
		idc := r.u(8)
		if idc == 255 {
			s.SARNum, s.SARDen = r.u(16), r.u(16)
		} else if int(idc) < len(sarTable) {
			s.SARNum, s.SARDen = sarTable[idc][0], sarTable[idc][1]
		}
	}
	if r.flag() { // overscan_info_present_flag
		r.u(1)
	}
	if r.flag() { // video_signal_type_present_flag
		s.VideoFormat = int(r.u(3))
		s.FullRange = r.flag()
		if r.flag() {
			s.HasColour = true
			s.Primaries, s.Transfer, s.Matrix = r.u(8), r.u(8), r.u(8)
		}
	}
	if r.flag() { // chroma_loc_info_present_flag
		r.ue()
		r.ue()
	}
	if r.flag() { // timing_info_present_flag
		s.NumUnits, s.TimeScale = r.u(32), r.u(32)
		s.FixedRate = r.flag()
		s.HasTiming = r.err == nil
	}
	nalHRD := r.flag()
	if nalHRD {
		s.HRDBitrate, s.HRDCBR = parseHRD(r)
	}
	vclHRD := r.flag()
	if vclHRD {
		parseHRD(r)
	}
	if nalHRD || vclHRD {
		r.u(1) // low_delay_hrd_flag
	}
	r.u(1)        // pic_struct_present_flag
	if r.flag() { // bitstream_restriction_flag
		r.u(1)
		r.ue()
		r.ue()
		r.ue()
		r.ue()
		s.MaxReorder = r.ue()
		s.MaxDecBuffer = r.ue()
		s.HasReorder = r.err == nil
	}
}

func parseHRD(r *bitReader) (bitrate uint64, cbr bool) {
	cnt := r.ue() + 1
	scale := r.u(4)
	r.u(4) // cpb_size_scale
	for i := uint64(0); i < cnt && i < 32 && r.err == nil; i++ {
		v := r.ue() + 1
		r.ue()
		c := r.flag()
		if i == 0 {
			bitrate, cbr = v<<(6+scale), c
		}
	}
	r.u(5)
	r.u(5)
	r.u(5)
	r.u(5)
	return bitrate, cbr
}

var sarTable = [][2]uint64{
	{0, 0}, {1, 1}, {12, 11}, {10, 11}, {16, 11}, {40, 33}, {24, 11}, {20, 11}, {32, 11},
	{80, 33}, {18, 11}, {15, 11}, {64, 33}, {160, 99}, {4, 3}, {3, 2}, {2, 1},
}

// parsePPSEntropy returns "CABAC" or "CAVLC" for a PPS NAL unit.
func parsePPSEntropy(nal []byte) (string, error) {
	if len(nal) < 2 || nal[0]&0x1f != 8 {
		return "", errors.New("not a PPS NAL unit")
	}
	r := &bitReader{b: rbsp(nal[1:])}
	r.ue() // pic_parameter_set_id
	r.ue() // seq_parameter_set_id
	cabac := r.flag()
	if r.err != nil {
		return "", r.err
	}
	if cabac {
		return "CABAC", nil
	}
	return "CAVLC", nil
}

func h264ProfileName(profile, constraints uint8) string {
	cs := func(n uint) bool { return constraints&(0x80>>n) != 0 }
	switch profile {
	case 66:
		if cs(1) {
			return "Constrained Baseline"
		}
		return "Baseline"
	case 77:
		return "Main"
	case 88:
		return "Extended"
	case 100:
		switch {
		case cs(4) && cs(5):
			return "Constrained High"
		case cs(4):
			return "Progressive High"
		}
		return "High"
	case 110:
		if cs(3) {
			return "High 10 Intra"
		}
		return "High 10"
	case 122:
		return "High 4:2:2"
	case 244:
		return "High 4:4:4 Predictive"
	case 44:
		return "CAVLC 4:4:4 Intra"
	}
	return fmt.Sprintf("profile %d", profile)
}

func h264LevelName(profile, constraints, level uint8) string {
	if level == 9 || (level == 11 && constraints&0x10 != 0 && (profile == 66 || profile == 77 || profile == 88)) {
		return "1b"
	}
	if level%10 == 0 {
		return fmt.Sprintf("%d", level/10)
	}
	return fmt.Sprintf("%d.%d", level/10, level%10)
}

// describeProfileLevelID decodes an SDP profile-level-id such as "42e01f".
func describeProfileLevelID(plid string) string {
	b, err := hex.DecodeString(plid)
	if err != nil || len(b) != 3 {
		return ""
	}
	return h264ProfileName(b[0], b[1]) + " " + h264LevelName(b[0], b[1], b[2])
}

// fmtpValue returns a parameter from an SDP fmtp line.
func fmtpValue(fmtp, key string) string {
	for _, kv := range strings.Split(fmtp, ";") {
		k, v, ok := strings.Cut(strings.TrimSpace(kv), "=")
		if ok && strings.EqualFold(k, key) {
			return v
		}
	}
	return ""
}

// h264Levels lists MaxMBPS and MaxFS per level (ITU-T H.264 Table A-1).
var h264Levels = []struct {
	idc         uint8
	name        string
	maxMBPS     float64
	maxFrameMBs uint64
}{
	{10, "1", 1485, 99}, {11, "1.1", 3000, 396}, {12, "1.2", 6000, 396}, {13, "1.3", 11880, 396},
	{20, "2", 11880, 396}, {21, "2.1", 19800, 792}, {22, "2.2", 20250, 1620},
	{30, "3", 40500, 1620}, {31, "3.1", 108000, 3600}, {32, "3.2", 216000, 5120},
	{40, "4", 245760, 8192}, {41, "4.1", 245760, 8192}, {42, "4.2", 522240, 8704},
	{50, "5", 589824, 22080}, {51, "5.1", 983040, 36864}, {52, "5.2", 2073600, 36864},
	{60, "6", 4177920, 139264}, {61, "6.1", 8355840, 139264}, {62, "6.2", 16711680, 139264},
}

// levelCheck reports whether the frame size and rate fit the signalled level.
// It returns an empty string when they fit or the level is unknown.
func levelCheck(s *spsInfo, fps float64) string {
	if fps <= 0 {
		return ""
	}
	frameMBs := s.MbWidth * s.MbHeight
	mbps := float64(frameMBs) * fps
	idx := -1
	for i, l := range h264Levels {
		if l.idc == s.Level {
			idx = i
		}
	}
	if idx < 0 || (frameMBs <= h264Levels[idx].maxFrameMBs && mbps <= h264Levels[idx].maxMBPS*1.01) {
		return ""
	}
	need := "none"
	for _, l := range h264Levels {
		if frameMBs <= l.maxFrameMBs && mbps <= l.maxMBPS*1.01 {
			need = l.name
			break
		}
	}
	return fmt.Sprintf("%dx%d at %.2f fps exceeds H.264 Level %s limits (frame %d MBs, max %d; %.0f MB/s, max %.0f); Level %s or higher is needed",
		s.Width, s.Height, fps, h264Levels[idx].name, frameMBs, h264Levels[idx].maxFrameMBs, mbps, h264Levels[idx].maxMBPS, need)
}

func colourName(kind string, v uint64) string {
	names := map[string]map[uint64]string{
		"primaries": {1: "BT.709", 5: "BT.601-625", 6: "BT.601-525", 9: "BT.2020"},
		"transfer":  {1: "BT.709", 6: "BT.601", 13: "sRGB", 16: "PQ", 18: "HLG"},
		"matrix":    {0: "RGB", 1: "BT.709", 5: "BT.601-625", 6: "BT.601-525", 9: "BT.2020nc"},
	}
	if n, ok := names[kind][v]; ok {
		return n
	}
	return fmt.Sprintf("%d", v)
}

// describe returns human-readable lines for the parsed SPS.
func (s *spsInfo) describe(entropy string) []string {
	chroma := map[uint64]string{0: "4:0:0", 1: "4:2:0", 2: "4:2:2", 3: "4:4:4"}[s.ChromaFormat]
	scan := "progressive"
	if !s.FrameMbsOnly {
		scan = "interlaced (field/MBAFF coding)"
	}
	size := fmt.Sprintf("%dx%d", s.Width, s.Height)
	if s.Cropped {
		size += fmt.Sprintf(" (coded %dx%d)", s.MbWidth*16, s.MbHeight*16)
	}
	l1 := fmt.Sprintf("H.264 %s, Level %s, %s, %s %d-bit, %s, ref frames %d, POC type %d",
		h264ProfileName(s.Profile, s.Constraints), h264LevelName(s.Profile, s.Constraints, s.Level),
		size, chroma, 8+s.BitDepthLuma, scan, s.MaxRefFrames, s.POCType)
	if entropy != "" {
		l1 += ", " + entropy
	}
	lines := []string{l1}
	if !s.VUI {
		return append(lines, "VUI: absent (no frame rate, aspect ratio or colour information in the stream)")
	}
	var v []string
	if fps := s.FPS(); fps > 0 {
		fixed := "variable"
		if s.FixedRate {
			fixed = "fixed"
		}
		v = append(v, fmt.Sprintf("%.2f fps (%s)", fps, fixed))
	} else {
		v = append(v, "no timing info")
	}
	if s.SARNum > 0 && s.SARDen > 0 {
		v = append(v, fmt.Sprintf("SAR %d:%d", s.SARNum, s.SARDen))
	}
	if s.VideoFormat >= 0 {
		rng := "limited range"
		if s.FullRange {
			rng = "full range"
		}
		v = append(v, rng)
	}
	if s.HasColour {
		v = append(v, fmt.Sprintf("colour %s/%s/%s", colourName("primaries", s.Primaries),
			colourName("transfer", s.Transfer), colourName("matrix", s.Matrix)))
	}
	if s.HRDBitrate > 0 {
		mode := "VBR"
		if s.HRDCBR {
			mode = "CBR"
		}
		v = append(v, fmt.Sprintf("HRD %d kbps %s", s.HRDBitrate/1000, mode))
	}
	if s.HasReorder {
		if s.MaxReorder == 0 {
			v = append(v, "no frame reordering (no B-frames)")
		} else {
			v = append(v, fmt.Sprintf("frame reordering up to %d (B-frames)", s.MaxReorder))
		}
	}
	return append(lines, "VUI: "+strings.Join(v, ", "))
}

// h264NAL is one NAL unit (or the start of a fragmented one) found in an RTP payload.
type h264NAL struct {
	typ  uint8
	data []byte // complete NAL unit; nil for an FU-A fragment
}

// h264PayloadNALs lists the NAL units carried by an H.264 RTP payload (RFC 6184).
// FU-A fragments are reported only for the first fragment, without data.
func h264PayloadNALs(p []byte) []h264NAL {
	if len(p) < 1 {
		return nil
	}
	switch t := p[0] & 0x1f; t {
	case 24: // STAP-A
		var out []h264NAL
		for i := 1; i+2 <= len(p); {
			size := int(p[i])<<8 | int(p[i+1])
			i += 2
			if size == 0 || i+size > len(p) {
				break
			}
			out = append(out, h264NAL{typ: p[i] & 0x1f, data: p[i : i+size]})
			i += size
		}
		return out
	case 28: // FU-A
		if len(p) >= 2 && p[1]&0x80 != 0 {
			return []h264NAL{{typ: p[1] & 0x1f}}
		}
		return nil
	default:
		if t >= 1 && t <= 23 {
			return []h264NAL{{typ: t, data: p}}
		}
		return nil
	}
}
