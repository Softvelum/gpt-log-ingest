# Whisper process removed after connection timeout

Product: Nimble Streamer
Component: Whisper transcription
Logging function: log_i
Log level: info
Documentation ID: log_i-d3af0d96e88e78a7

## Log pattern

```text
whisper transcriber removed on connection timeout for [%s] lang=%s
```

## Parameters

- Argument 1 — Stream: Application and stream name.
- Argument 2 — Language: Configured language identifier for this transcription process; it may differ from the effective detected language.

## Log examples

Illustrative examples; these are not captured server logs.

```text
whisper transcriber removed on connection timeout for [live/news] lang=en
```

## Short description

Whisper process removed after connection timeout.

## Trigger condition

More than two seconds have elapsed since process creation and its connection has not been attached.

## Explanation

The unconnected transcription process is removed. Cleanup waits briefly for it to stop and can terminate it.

## Possible causes

- More than two seconds have elapsed since process creation and its connection has not been attached.

## What to check

1. Check child startup errors and local connection messages.

## Related log messages

- [Whisper child process started](log_d-16053848967c8f4e.md)
- [Whisper process connected](log_i-b4b665778f8c455b.md)
