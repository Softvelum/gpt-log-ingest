# Audio decoder worker started

Product: Nimble Streamer
Component: Audio decoding
Logging function: log_d
Log level: debug
Documentation ID: log_d-6584def2ee485fb7

## Log pattern

```text
AudioDecoderThread(%s) started
```

## Parameters

- Argument 1 — Input: Live application/stream name or file-track display name, text; may be empty if no source was selected.

## Log examples

Illustrative examples; these are not captured server logs.

```text
AudioDecoderThread(live/camera) started
```

## Short description

Audio decoder worker started.

## Trigger condition

The audio worker enters its processing loop after deriving its input name.

## Explanation

Source and output updates and audio decoding follow; this message does not confirm successful decoder initialization.
