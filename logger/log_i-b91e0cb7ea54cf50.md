# H.264 hardware decoder is unavailable in FFmpeg

Product: Nimble Streamer
Component: Video decoding
Logging function: log_i
Log level: info
Documentation ID: log_i-b91e0cb7ea54cf50

## Log pattern

```text
h264_cuvid decoder is unavailable, using default decoder for [%s]
```

## Parameters

- Argument 1 — Stream: Input stream name, text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
h264_cuvid decoder is unavailable, using default decoder for [live/camera]
```

## Short description

H.264 hardware decoder is unavailable in FFmpeg.

## Trigger condition

The NVENC-ffmpeg option is selected but FFmpeg cannot find the h264_cuvid decoder.

## Explanation

The worker constructs the default H.264 decoder and continues.

## Possible causes

- The loaded FFmpeg codec library does not register the requested hardware decoder.

## What to check

1. Check the selected decoder option and the loaded FFmpeg library capabilities.

## Recommended action

Use a codec library providing h264_cuvid if that backend is required; otherwise allow the default decoder.
