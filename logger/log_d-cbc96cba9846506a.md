# Encoder worker stopped

Product: Nimble Streamer
Component: Transcoding output
Logging function: log_d
Log level: debug
Documentation ID: log_d-cbc96cba9846506a

## Log pattern

```text
EncoderThread(%s) stopped
```

## Parameters

- Argument 1 — Output: Output application/stream name, text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
EncoderThread(live/encoded) stopped
```

## Short description

Encoder worker stopped.

## Trigger condition

The output worker leaves its loop after a stop request, settings-update failure, or media-processing failure.

## Explanation

Audio and video encoders are released, and any local device writer is stopped and released before the message.
