# DVR export movie metadata builder is unavailable

Product: Nimble Streamer
Component: DVR export
Logging function: log_e
Log level: error
Documentation ID: log_e-d08cfa202b078bee

## Log pattern

```text
failed to create moov builder, stream=%s utc_start=%lld utc_end=%lld
```

## Parameters

- Argument 1 — Stream: Text naming the application and stream.
- Argument 2 — UTC start: 64-bit requested UTC start in seconds since the Unix epoch; zero leaves this bound unspecified.
- Argument 3 — UTC end: 64-bit requested UTC end in seconds since the Unix epoch; zero leaves this bound unspecified.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to create moov builder, stream=live/camera utc_start=1700000000 utc_end=1700000060
```

## Short description

DVR export movie metadata builder is unavailable.

## Trigger condition

The video or audio export setup obtains no movie-metadata builder.

## Explanation

An HTTP 500 response is requested.

## Possible causes

- The video or audio export setup obtains no movie-metadata builder.

## What to check

1. Check allocation failures while preparing the export.

## Recommended action

Check allocation failures while preparing the export.
