# Alternate OCR training data selected

Product: Nimble Streamer
Component: DVB subtitle text recognition
Logging function: log_d
Log level: debug
Documentation ID: log_d-6ebfa55826d12fab

## Log pattern

```text
using trained data %s for lang=%s path=%s stream=%s
```

## Parameters

- Argument 1 — Training data: Successfully selected alternate training-data language name.
- Argument 2 — Language: Lowercase DVB subtitle language code.
- Argument 3 — Directory: Configured Tesseract training-data directory.
- Argument 4 — Stream: Application and stream name.

## Log examples

Illustrative examples; these are not captured server logs.

```text
using trained data deu for lang=ger path=/data/tessdata stream=live/news
```

## Short description

Alternate OCR training data selected.

## Trigger condition

Direct initialization by subtitle language failed but initialization with its alternate training-data name succeeded.

## Explanation

Recognition continues using that training-data mapping.

## Possible causes

- Direct initialization by subtitle language failed but initialization with its alternate training-data name succeeded.

## What to check

1. Confirm the mapped training-data language is appropriate for the subtitle language.
