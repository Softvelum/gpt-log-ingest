# Video decoder worker started

Product: Nimble Streamer
Component: Video decoding
Logging function: log_d
Log level: debug
Documentation ID: log_d-b6624b1f7ab890ac

## Log pattern

```text
VideoDecoderThread(%s) started
```

## Parameters

- Argument 1 — Input: Live application/stream name or file-track display name, text; may be empty if no source was selected.

## Log examples

Illustrative examples; these are not captured server logs.

```text
VideoDecoderThread(live/camera) started
```

## Short description

Video decoder worker started.

## Trigger condition

The video worker enters its processing loop after deriving its live or file input name.

## Explanation

Source updates and decoder selection follow; this message does not confirm decoder readiness.
