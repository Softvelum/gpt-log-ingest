# DASH audio initialization exceeds its buffer

Product: Nimble Streamer
Component: HLS and DASH packaging
Logging function: log_e
Log level: error
Documentation ID: log_e-a07241af4f1bcc5b

## Log pattern

```text
DashLiveSegmentList::generate_audio_initialization_segment failed to generate init segment.aac_codec_info_size=%d
```

## Parameters

- Argument 1 — Configuration size: Supplied decoder-configuration byte count; the calculated complete initialization segment also includes container overhead.

## Log examples

Illustrative examples; these are not captured server logs.

```text
DashLiveSegmentList::generate_audio_initialization_segment failed to generate init segment.aac_codec_info_size=32768
```

## Short description

DASH audio initialization exceeds its buffer.

## Trigger condition

The calculated audio initialization-segment size exceeds the fixed 10,240-byte buffer.

## Explanation

A fallback initialization segment is generated with zero codec-configuration bytes. Normal initialization with the supplied configuration is not produced.

## Possible causes

- The calculated audio initialization-segment size exceeds the fixed 10,240-byte buffer.

## What to check

1. Check the codec-configuration size and, when applicable, encryption initialization data.

## Recommended action

Correct oversized or malformed decoder configuration before packaging.
