# DVB subtitle decoding failed

Product: Nimble Streamer
Component: Caption and subtitle handling
Logging function: log_e
Log level: error
Documentation ID: log_e-c3efe3bc6ff27cf3

## Log pattern

```text
failed to decode dvb subtitle for [%s]: %s
```

## Parameters

- Argument 1 — Stream: Application and stream name.
- Argument 2 — Error: Media-library error description.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to decode dvb subtitle for [live/news]: Invalid data found when processing input
```

## Short description

DVB subtitle decoding failed.

## Trigger condition

The media library returns a negative result while decoding the subtitle packet.

## Explanation

No OCR request is queued for this packet and subtitle decoding reports failure.

## Possible causes

- The media library returns a negative result while decoding the subtitle packet.

## What to check

1. Check the input DVB subtitle payload and decoder error text.
