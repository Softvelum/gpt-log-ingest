# Whisper audio arrived without configuration

Product: Nimble Streamer
Component: Whisper transcription
Logging function: log_e
Log level: error
Documentation ID: log_e-d0fc9aef952c4bd1

## Log pattern

```text
failed to get audio config for [%s] lang=%s
```

## Parameters

- Argument 1 — Stream: Application and stream name.
- Argument 2 — Language: Configured language identifier for this transcription process; it may differ from the effective detected language.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to get audio config for [live/news] lang=en
```

## Short description

Whisper audio arrived without configuration.

## Trigger condition

Audio data is received before a valid audio configuration is available and no decoder exists.

## Explanation

Processing fails before a Whisper decoder is created.

## Possible causes

- Audio data is received before a valid audio configuration is available and no decoder exists.

## What to check

1. Check that valid audio configuration is delivered before audio data.

## Related log messages

- [Invalid Whisper audio configuration](log_e-21d9151b5cd9381d.md)
