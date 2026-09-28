# NVIDIA AV1 decoding falls back to FFmpeg

Product: Nimble Streamer
Component: Video decoding
Logging function: log_i
Log level: info
Documentation ID: log_i-ff0f42242eee3f36

## Log pattern

```text
nvidia av1 decoder is unavailable, using default decoder for [%s]
```

## Parameters

- Argument 1 — Stream: Input stream name, text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
nvidia av1 decoder is unavailable, using default decoder for [live/camera]
```

## Short description

NVIDIA AV1 decoding falls back to FFmpeg.

## Trigger condition

Native NVIDIA AV1 decoding is requested but NVIDIA decode libraries are not loaded.

## Explanation

The default FFmpeg AV1 decoder is constructed and processing continues.

## Possible causes

- The optional NVIDIA decoding libraries were unavailable during loading.

## What to check

1. Check earlier NVIDIA library-loading messages if GPU decoding is required.

## Recommended action

Restore the required libraries or allow the default AV1 decoder.
