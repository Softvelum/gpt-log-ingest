# Encoder worker started

Product: Nimble Streamer
Component: Transcoding output
Logging function: log_d
Log level: debug
Documentation ID: log_d-bcf8be0b2a6156d6

## Log pattern

```text
EncoderThread(%s) started
```

## Parameters

- Argument 1 — Output: Output application/stream name, text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
EncoderThread(live/encoded) started
```

## Short description

Encoder worker started.

## Trigger condition

The output worker enters its run loop.

## Explanation

It begins periodic source and encoder updates and media processing; this message precedes successful codec initialization.
