# AVC output frame buffer write failed

Product: Nimble Streamer
Component: MPEG-TS video conversion
Logging function: log_e
Log level: error
Documentation ID: log_e-fae732e8c6e8b312

## Log pattern

```text
MPEG2TSLiveStream::processAVCVideoMessage frame buffer too small
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
MPEG2TSLiveStream::processAVCVideoMessage frame buffer too small
```

## Short description

AVC output frame buffer write failed.

## Trigger condition

Converting an AVC Annex B frame cannot append a four-byte NAL length or the associated SPS, PPS, or other NAL data.

## Explanation

The converter returns false before delivering the completed converted frame. This buffer has a 10 MiB maximum. In direct frame processing or pending-frame flush, failure resets video initialization, timestamps, parameter sets, and assembly buffers; this message alone does not stop the network input.

## Possible causes

- The converted frame buffer could not store the required length or NAL bytes.

## What to check

1. Check frame size and allocation diagnostics against the conversion buffer limit.

## Recommended action

Correct oversized or malformed input frames or resolve allocation failure.
