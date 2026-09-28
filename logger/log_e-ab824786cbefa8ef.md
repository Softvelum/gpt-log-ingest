# Fragmented MP4 HLS AVC initialization exceeds its buffer

Product: Nimble Streamer
Component: Live stream codec and packaging setup
Logging function: log_e
Log level: error
Documentation ID: log_e-ab824786cbefa8ef

## Log pattern

```text
Fmp4LiveSegmentList::get_cenc_video_initialization_segment_size failed to generate init segment for [%s/%s] avc_codec_info_size=%d
```

## Parameters

- Argument 1 — Application: Application containing the affected live stream.
- Argument 2 — Stream: Live stream name.
- Argument 3 — Configuration size: Supplied decoder-configuration size in bytes; total initialization size also includes MP4 overhead.

## Log examples

Illustrative examples; these are not captured server logs.

```text
Fmp4LiveSegmentList::get_cenc_video_initialization_segment_size failed to generate init segment for [live/camera] avc_codec_info_size=16384
```

## Short description

Fragmented MP4 HLS AVC initialization exceeds its buffer.

## Trigger condition

The calculated AVC initialization-segment size exceeds the fixed 8,192-byte buffer. CENC initialization is being prepared.

## Explanation

A fallback unencrypted initialization segment with zero codec-configuration bytes is generated and queued for the HLS track. This fallback does not preserve requested CENC initialization.

## Possible causes

- The calculated AVC initialization-segment size exceeds the fixed 8,192-byte buffer. CENC initialization is being prepared.

## What to check

1. Check decoder-configuration size and any encryption initialization overhead.

## Recommended action

Correct oversized or malformed initialization data before packaging.
