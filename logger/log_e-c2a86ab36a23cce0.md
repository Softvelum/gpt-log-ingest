# DASH AVC initialization exceeds its buffer

Product: Nimble Streamer
Component: HLS and DASH packaging
Logging function: log_e
Log level: error
Documentation ID: log_e-c2a86ab36a23cce0

## Log pattern

```text
DashLiveSegmentList::generate_h264_initialization_segment failed to generate init segment.avc_codec_info_size=%d
```

## Parameters

- Argument 1 — Configuration size: Supplied decoder-configuration byte count; the calculated complete initialization segment also includes container overhead.

## Log examples

Illustrative examples; these are not captured server logs.

```text
DashLiveSegmentList::generate_h264_initialization_segment failed to generate init segment.avc_codec_info_size=32768
```

## Short description

DASH AVC initialization exceeds its buffer.

## Trigger condition

The calculated AVC initialization-segment size exceeds the fixed 20,480-byte buffer.

## Explanation

A fallback initialization segment is generated with zero codec-configuration bytes. Normal initialization with the supplied configuration is not produced.

## Possible causes

- The calculated AVC initialization-segment size exceeds the fixed 20,480-byte buffer.

## What to check

1. Check the codec-configuration size and, when applicable, encryption initialization data.

## Recommended action

Correct oversized or malformed decoder configuration before packaging.
