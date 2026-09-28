# Unsupported transcription language replaced with English

Product: Nimble Streamer
Component: Live speech transcription
Logging function: log_e
Log level: error
Documentation ID: log_e-1aabb00d06e4628f

## Log pattern

```text
unsupported transcriber lang='%s' changed to 'en' for [%s]
```

## Parameters

- Argument 1 — Language: Requested source language code.
- Argument 2 — Stream: Configured stream name.

## Log examples

Illustrative examples; these are not captured server logs.

```text
unsupported transcriber lang='zz' changed to 'en' for [live/news]
```

## Short description

Unsupported transcription language replaced with English.

## Trigger condition

The configured source language is absent from the selected transcription service language map.

## Explanation

Live speech transcription uses English and its subtitle language code for this stream. This path is available when speech-service support is enabled.

## Possible causes

- The configured source language is absent from the selected transcription service language map.

## What to check

1. Check the source language against the languages accepted by the selected service.

## Recommended action

Choose a supported source language if English is not intended.
