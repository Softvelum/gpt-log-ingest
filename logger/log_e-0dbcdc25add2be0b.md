# Whisper audio processing failed

Product: Nimble Streamer
Component: Whisper transcription
Logging function: log_e
Log level: error
Documentation ID: log_e-0dbcdc25add2be0b

## Log pattern

```text
failed to transcribe audio stream='%s' lang=%s
```

## Parameters

- Argument 1 — Stream: Application and stream name.
- Argument 2 — Language: Configured language identifier for this transcription process; it may differ from the effective detected language.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to transcribe audio stream='live/news' lang=en
```

## Short description

Whisper audio processing failed.

## Trigger condition

The transcription decoder cannot process the current encoded audio payload.

## Explanation

The incoming audio message returns failure. Causes include unavailable model initialization or failure to decode and resample audio.

## Possible causes

- The transcription decoder cannot process the current encoded audio payload.

## What to check

1. Check preceding model initialization and audio decoder diagnostics.

## Related log messages

- [Whisper model context creation failed](log_e-739a98e1fcc969d8.md)
