# Inactive Whisper process removed

Product: Nimble Streamer
Component: Whisper transcription
Logging function: log_i
Log level: info
Documentation ID: log_i-ccc777ff10c2723b

## Log pattern

```text
whisper transcriber removed on inactivity for [%s] lang=%s
```

## Parameters

- Argument 1 — Stream: Application and stream name.
- Argument 2 — Language: Configured language identifier for this transcription process; it may differ from the effective detected language.

## Log examples

Illustrative examples; these are not captured server logs.

```text
whisper transcriber removed on inactivity for [live/news] lang=en
```

## Short description

Inactive Whisper process removed.

## Trigger condition

Three inactivity checks pass without a message-send attempt resetting the stream counter; checks run about every five seconds.

## Explanation

The socket association and transcription stream are removed, allowing process cleanup. Later audio may create a new process.

## Possible causes

- Three inactivity checks pass without a message-send attempt resetting the stream counter; checks run about every five seconds.

## What to check

1. Check whether audio is still supplied for the stream and language.

## Related log messages

- [Whisper transcription stream created](log_i-91321f67e0793402.md)
