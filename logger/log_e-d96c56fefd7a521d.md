# Cannot create Whisper child process

Product: Nimble Streamer
Component: Whisper transcription
Logging function: log_e
Log level: error
Documentation ID: log_e-d96c56fefd7a521d

## Log pattern

```text
failed to create whisper transcriber for [%s] lang=%s
```

## Parameters

- Argument 1 — Stream: Application and stream name.
- Argument 2 — Language: Configured language identifier for this transcription process; it may differ from the effective detected language.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to create whisper transcriber for [live/news] lang=en
```

## Short description

Cannot create Whisper child process.

## Trigger condition

Allocating the transcription stream or starting its child process fails for a requested language.

## Explanation

That language is skipped for this audio dispatch; other languages are still attempted.

## Possible causes

- Allocating the transcription stream or starting its child process fails for a requested language.

## What to check

1. Check the child executable and preceding process-start diagnostics.
