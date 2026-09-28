# HEVC hardware decoder is unavailable in FFmpeg

Product: Nimble Streamer
Component: Video decoding
Logging function: log_i
Log level: info
Documentation ID: log_i-7ebaf625c8c0eda2

## Log pattern

```text
hevc_cuvid decoder is unavailable, using default decoder for [%s]
```

## Parameters

- Argument 1 — Stream: Input stream name, text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
hevc_cuvid decoder is unavailable, using default decoder for [live/camera]
```

## Short description

HEVC hardware decoder is unavailable in FFmpeg.

## Trigger condition

The NVENC-ffmpeg option is selected but FFmpeg cannot find the hevc_cuvid decoder.

## Explanation

The worker constructs the default HEVC decoder and continues.

## Possible causes

- The loaded FFmpeg codec library does not register the requested hardware decoder.

## What to check

1. Check the selected decoder option and the loaded FFmpeg library capabilities.

## Recommended action

Use a codec library providing hevc_cuvid if that backend is required; otherwise allow the default decoder.
