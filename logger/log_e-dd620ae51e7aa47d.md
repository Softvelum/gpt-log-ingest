# Cannot queue Whisper audio data

Product: Nimble Streamer
Component: Whisper transcription
Logging function: log_e
Log level: error
Documentation ID: log_e-dd620ae51e7aa47d

## Log pattern

```text
failed to send whisper audio data msg for [%s] lang=%s
```

## Parameters

- Argument 1 — Stream: Application and stream name.
- Argument 2 — Language: Configured language identifier for this transcription process; it may differ from the effective detected language.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to send whisper audio data msg for [live/news] lang=en
```

## Short description

Cannot queue Whisper audio data.

## Trigger condition

The audio data message cannot be written to the child-process output buffer or output notification cannot be enabled.

## Explanation

The affected language stream and socket association are removed. Processing continues for any other language streams.

## Possible causes

- The audio data message cannot be written to the child-process output buffer or output notification cannot be enabled.

## What to check

1. Check child connection health and whether queued audio has filled the output buffer.
