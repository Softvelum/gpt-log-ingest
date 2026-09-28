# Invalid transcription configuration JSON

Product: Nimble Streamer
Component: Live speech transcription
Logging function: log_e
Log level: error
Documentation ID: log_e-28e2ef0b6badf6a3

## Log pattern

```text
invalid transcription_config for stream=[%s]
```

## Parameters

- Argument 1 — Stream: Configured stream name.

## Log examples

Illustrative examples; these are not captured server logs.

```text
invalid transcription_config for stream=[live/news]
```

## Short description

Invalid transcription configuration JSON.

## Trigger condition

The optional transcription configuration cannot be parsed as JSON while preparing recognition.

## Explanation

Recognition continues with a new configuration containing the selected language. The invalid custom configuration is not used.

## Possible causes

- The optional transcription configuration cannot be parsed as JSON while preparing recognition.

## What to check

1. Check the custom transcription configuration JSON.

## Recommended action

Correct its JSON syntax to retain the intended service options.
