package main

import (
	"encoding/binary"
	"math"
)

// A minimal live Matroska muxer: unknown-size Segment and Clusters with
// SimpleBlocks, enough for ffplay/ffmpeg/VLC/mpv to play from a pipe or file.

const (
	idEBML              = 0x1A45DFA3
	idEBMLVersion       = 0x4286
	idEBMLReadVersion   = 0x42F7
	idEBMLMaxIDLength   = 0x42F2
	idEBMLMaxSizeLength = 0x42F3
	idDocType           = 0x4282
	idDocTypeVersion    = 0x4287
	idDocTypeReadVer    = 0x4285
	idSegment           = 0x18538067
	idInfo              = 0x1549A966
	idTimestampScale    = 0x2AD7B1
	idMuxingApp         = 0x4D80
	idWritingApp        = 0x5741
	idTracks            = 0x1654AE6B
	idTrackEntry        = 0xAE
	idTrackNumber       = 0xD7
	idTrackUID          = 0x73C5
	idTrackType         = 0x83
	idFlagLacing        = 0x9C
	idCodecID           = 0x86
	idCodecPrivate      = 0x63A2
	idCodecDelay        = 0x56AA
	idSeekPreRoll       = 0x56BB
	idVideo             = 0xE0
	idPixelWidth        = 0xB0
	idPixelHeight       = 0xBA
	idAudio             = 0xE1
	idSamplingFrequency = 0xB5
	idChannels          = 0x9F
	idCluster           = 0x1F43B675
	idTimestamp         = 0xE7
	idSimpleBlock       = 0xA3
)

var unknownSize = []byte{0x01, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF}

func ebmlID(id uint32) []byte {
	switch {
	case id >= 1<<24:
		return []byte{byte(id >> 24), byte(id >> 16), byte(id >> 8), byte(id)}
	case id >= 1<<16:
		return []byte{byte(id >> 16), byte(id >> 8), byte(id)}
	case id >= 1<<8:
		return []byte{byte(id >> 8), byte(id)}
	}
	return []byte{byte(id)}
}

// ebmlSize encodes a data size as a variable-length integer.
func ebmlSize(n uint64) []byte {
	for l := 1; l <= 8; l++ {
		if n < 1<<(7*uint(l))-1 {
			b := make([]byte, l)
			for i := l - 1; i >= 0; i-- {
				b[i] = byte(n)
				n >>= 8
			}
			b[0] |= 0x80 >> uint(l-1)
			return b
		}
	}
	panic("ebml size too large")
}

func ebmlElem(id uint32, data ...[]byte) []byte {
	var body []byte
	for _, d := range data {
		body = append(body, d...)
	}
	out := append(ebmlID(id), ebmlSize(uint64(len(body)))...)
	return append(out, body...)
}

func ebmlUint(id uint32, v uint64) []byte {
	n := 1
	for n < 8 && v>>(8*uint(n)) != 0 {
		n++
	}
	b := make([]byte, n)
	for i := n - 1; i >= 0; i-- {
		b[i] = byte(v)
		v >>= 8
	}
	return ebmlElem(id, b)
}

func ebmlString(id uint32, s string) []byte { return ebmlElem(id, []byte(s)) }

func ebmlFloat(id uint32, f float64) []byte {
	b := make([]byte, 8)
	binary.BigEndian.PutUint64(b, math.Float64bits(f))
	return ebmlElem(id, b)
}

// mkvTrack describes one output track.
type mkvTrack struct {
	number        uint64
	video         bool
	codecID       string
	codecPrivate  []byte
	width, height int
	sampleRate    float64
	channels      uint64
}

// mkvHeader returns the EBML header, the start of a live Segment, Info and Tracks.
func mkvHeader(tracks []mkvTrack) []byte {
	out := ebmlElem(idEBML,
		ebmlUint(idEBMLVersion, 1), ebmlUint(idEBMLReadVersion, 1),
		ebmlUint(idEBMLMaxIDLength, 4), ebmlUint(idEBMLMaxSizeLength, 8),
		ebmlString(idDocType, "matroska"), ebmlUint(idDocTypeVersion, 4), ebmlUint(idDocTypeReadVer, 2))
	out = append(out, ebmlID(idSegment)...)
	out = append(out, unknownSize...)
	out = append(out, ebmlElem(idInfo,
		ebmlUint(idTimestampScale, 1000000), // milliseconds
		ebmlString(idMuxingApp, "whep-check"), ebmlString(idWritingApp, "whep-check"))...)
	var entries [][]byte
	for _, t := range tracks {
		fields := [][]byte{
			ebmlUint(idTrackNumber, t.number), ebmlUint(idTrackUID, t.number),
			ebmlUint(idFlagLacing, 0), ebmlString(idCodecID, t.codecID),
		}
		if len(t.codecPrivate) > 0 {
			fields = append(fields, ebmlElem(idCodecPrivate, t.codecPrivate))
		}
		if t.video {
			fields = append(fields, ebmlUint(idTrackType, 1),
				ebmlElem(idVideo, ebmlUint(idPixelWidth, uint64(t.width)), ebmlUint(idPixelHeight, uint64(t.height))))
		} else {
			fields = append(fields, ebmlUint(idTrackType, 2),
				ebmlElem(idAudio, ebmlFloat(idSamplingFrequency, t.sampleRate), ebmlUint(idChannels, t.channels)))
			if t.codecID == "A_OPUS" {
				fields = append(fields, ebmlUint(idCodecDelay, 0), ebmlUint(idSeekPreRoll, 80000000))
			}
		}
		entries = append(entries, ebmlElem(idTrackEntry, fields...))
	}
	return append(out, ebmlElem(idTracks, entries...)...)
}

// mkvCluster starts a live Cluster at the given timestamp (ms).
func mkvCluster(tsMs int64) []byte {
	out := append(ebmlID(idCluster), unknownSize...)
	return append(out, ebmlUint(idTimestamp, uint64(tsMs))...)
}

// mkvSimpleBlock encodes one frame relative to the Cluster timestamp.
func mkvSimpleBlock(track uint64, rel int16, key bool, data []byte) []byte {
	hdr := append(ebmlSize(track), byte(uint16(rel)>>8), byte(uint16(rel)))
	flags := byte(0)
	if key {
		flags |= 0x80
	}
	hdr = append(hdr, flags)
	return ebmlElem(idSimpleBlock, hdr, data)
}

// avcDecoderConfig builds an AVCDecoderConfigurationRecord (ISO/IEC 14496-15).
func avcDecoderConfig(sps, pps []byte) []byte {
	b := []byte{1, sps[1], sps[2], sps[3], 0xFF, 0xE1}
	b = binary.BigEndian.AppendUint16(b, uint16(len(sps)))
	b = append(b, sps...)
	b = append(b, 1)
	b = binary.BigEndian.AppendUint16(b, uint16(len(pps)))
	return append(b, pps...)
}

// opusHead builds the OpusHead identification header (RFC 7845).
func opusHead(channels uint8, sampleRate uint32) []byte {
	b := append([]byte("OpusHead"), 1, channels)
	b = binary.LittleEndian.AppendUint16(b, 0) // pre-skip
	b = binary.LittleEndian.AppendUint32(b, sampleRate)
	b = binary.LittleEndian.AppendUint16(b, 0) // output gain
	return append(b, 0)                        // channel mapping family
}
