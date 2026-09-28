# No training data mapping for subtitle language

Product: Nimble Streamer
Component: DVB subtitle text recognition
Logging function: log_e
Log level: error
Documentation ID: log_e-022ab32db6f99ba8

## Log pattern

```text
failed to find tesseract traineddata for stream='%s' lang='%s' path='%s'
```

## Parameters

- Argument 1 — Stream: Application and stream name.
- Argument 2 — Language: Lowercase DVB subtitle language code.
- Argument 3 — Directory: Configured Tesseract training-data directory.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to find tesseract traineddata for stream='live/news' lang='zzz' path='/data/tessdata'
```

## Short description

No training data mapping for subtitle language.

## Trigger condition

Direct language initialization fails and language discovery yields no alternative training-data mapping.

## Explanation

The current subtitle message is skipped successfully, without creating an OCR decoder for this language.

## Possible causes

- Direct language initialization fails and language discovery yields no alternative training-data mapping.

## What to check

1. Check that training data is installed for the indicated subtitle language.

## Recommended action

Provide matching language training data in the configured directory.
