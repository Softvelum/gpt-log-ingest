# NVIDIA AV1 configuration record is invalid

Product: Nimble Streamer
Component: NVIDIA video decoding
Logging function: log_e
Log level: error
Documentation ID: log_e-b0d060df608bb7fc

## Log pattern

```text
invalid av1 config record for [%s]
```

## Parameters

- Argument 1 — Stream: Input stream name, text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
invalid av1 config record for [live/camera]
```

## Short description

NVIDIA AV1 configuration record is invalid.

## Trigger condition

AV1 parser setup receives fewer than four configuration bytes, or the high marker bit of the first byte is not set.

## Explanation

Video parser creation stops before passing the AV1 sequence data to the driver.

## Possible causes

- The input AV1 configuration record is truncated or lacks its required marker bit.

## What to check

1. Check the AV1 configuration supplied by the source.

## Recommended action

Provide a valid AV1 configuration record.
