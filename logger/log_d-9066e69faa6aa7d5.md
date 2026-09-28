# FFmpeg video decoder opened

Product: Nimble Streamer
Component: FFmpeg video decoding
Logging function: log_d
Log level: debug
Documentation ID: log_d-9066e69faa6aa7d5

## Log pattern

```text
created decoder for [%s] n='%s' ln='%s'
```

## Parameters

- Argument 1 — Stream: Input stream name, text.
- Argument 2 — Decoder name: FFmpeg short codec name, text.
- Argument 3 — Decoder description: FFmpeg long codec description, text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
created decoder for [live/camera] n='h264' ln='H.264 video decoder'
```

## Short description

FFmpeg video decoder opened.

## Trigger condition

Opening the selected FFmpeg video decoder succeeds.

## Explanation

The decoder retains the selected codec identifier and is ready to receive packets.
