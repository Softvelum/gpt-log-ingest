# Unsupported Whisper language uses detection

Product: Nimble Streamer
Component: Whisper transcription
Logging function: log_e
Log level: error
Documentation ID: log_e-e0feac6aeaacf1bd

## Log pattern

```text
unsupported transcriber lang='%s' changed to 'auto'
```

## Parameters

- Argument 1 — Language: Unsupported requested language code.

## Log examples

Illustrative examples; these are not captured server logs.

```text
unsupported transcriber lang='zz' changed to 'auto'
```

## Short description

Unsupported Whisper language uses detection.

## Trigger condition

The requested language is neither auto nor present in the supported language map.

## Explanation

The effective Whisper language becomes auto for automatic detection. The original identifier still labels the process connection and related logs.

## Possible causes

- The requested language is neither auto nor present in the supported language map.

## What to check

1. Check the requested transcription language.

## Recommended action

Use a supported language code or auto.
