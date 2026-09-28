# Audio message is too short

Product: Nimble Streamer
Component: Audio decoding
Logging function: log_e
Log level: error
Documentation ID: log_e-3309fab2f4eebca1

## Log pattern

```text
wrong message size %d, [%s/%s]
```

## Parameters

- Argument 1 — Size: Unsigned byte count printed in decimal; see occurrence differences below.
- Argument 2 — Application: Application name, text.
- Argument 3 — Stream: Stream name, text.

AAC, MP3, Speex, and ordinary audio dispatch require at least two total bytes; PCM requires at least one. AC3 and Opus decoding require total size greater than the RTMP header size and report the remaining payload size, whose unsigned subtraction can wrap for a truncated header. AC3/EAC3 and Opus forwarding apply the same header comparison but report total size. Other occurrences report total size.

## Log examples

Illustrative examples; these are not captured server logs.

```text
wrong message size 1, [live/camera]
```

## Short description

Audio message is too short.

## Trigger condition

An audio message fails the minimum-size check for its packet type.

## Explanation

The message is rejected and the active decoding or encoded-audio forwarding operation fails.

## Possible causes

- The input message is empty or truncated before the required audio header or payload.

## What to check

1. Check input message framing and the relevant packet header size.

## Recommended action

Correct the upstream audio packet framing and restart the affected stream.
