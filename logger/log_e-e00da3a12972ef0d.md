# Mapped OCR language initialization failed

Product: Nimble Streamer
Component: DVB subtitle text recognition
Logging function: log_e
Log level: error
Documentation ID: log_e-e00da3a12972ef0d

## Log pattern

```text
failed to init tesseract api, lang=%s trained_data=%s path=%s stream=%s
```

## Parameters

- Argument 1 — Language: Lowercase DVB subtitle language code.
- Argument 2 — Training data: Alternate language or training-data name attempted.
- Argument 3 — Directory: Configured Tesseract training-data directory.
- Argument 4 — Stream: Application and stream name.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to init tesseract api, lang=ger trained_data=deu path=/data/tessdata stream=live/news
```

## Short description

Mapped OCR language initialization failed.

## Trigger condition

Initialization using the discovered or alternate training-data language name fails.

## Explanation

The subtitle message fails and no OCR instance is installed for this language.

## Possible causes

- Initialization using the discovered or alternate training-data language name fails.

## What to check

1. Check the mapped training data and directory accessibility.
