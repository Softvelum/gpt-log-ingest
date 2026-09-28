# Transcription settings file does not exist

Product: Nimble Streamer
Component: Whisper transcription
Logging function: log_e
Log level: error
Documentation ID: log_e-b0ac458dea7a8948

## Log pattern

```text
failed to open transcriber config '%s' ec.message()='%s'
```

## Parameters

- Argument 1 — File: Configured transcription settings path.
- Argument 2 — Error: Filesystem error description from the existence check.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to open transcriber config '/data/transcription.json' ec.message()='Success'
```

## Short description

Transcription settings file does not exist.

## Trigger condition

The filesystem existence check for the configured settings path returns false.

## Explanation

No content is loaded from that path. The error text can report success when the file simply does not exist.

## Possible causes

- The filesystem existence check for the configured settings path returns false.

## What to check

1. Check the configured file path and the filesystem error text.

## Recommended action

Provide the intended readable transcription settings file.
