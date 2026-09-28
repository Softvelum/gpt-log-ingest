# AV1 Xilinx decoder falls back to FFmpeg

Product: Nimble Streamer
Component: Video decoding
Logging function: log_i
Log level: info
Documentation ID: log_i-ab5537332edfa8bf

## Log pattern

```text
xilinx-ffmpeg av1 decoder is unavailable, using default decoder for [%s]
```

## Parameters

- Argument 1 — Stream: Input stream name, text.

The lookup uses av1_ama on MA35D builds.

## Log examples

Illustrative examples; these are not captured server logs.

```text
xilinx-ffmpeg av1 decoder is unavailable, using default decoder for [live/camera]
```

## Short description

AV1 Xilinx decoder falls back to FFmpeg.

## Trigger condition

The xilinx-ffmpeg option is selected but the applicable AV1 decoder name cannot be found in FFmpeg.

## Explanation

Decoder selection proceeds and uses the default FFmpeg decoder if no subsequent hardware choice succeeds.

## Possible causes

- The loaded codec library does not register the requested accelerator decoder.

## What to check

1. Check the accelerator build and corresponding FFmpeg codec library.

## Recommended action

Load the accelerator-compatible library if this backend is required, or use the default decoder.
