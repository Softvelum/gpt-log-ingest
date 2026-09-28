# DVR export buffer allocation failed

Product: Nimble Streamer
Component: DVR export
Logging function: log_e
Log level: error
Documentation ID: log_e-4ff8d1e45c73ccdc

## Log pattern

```text
failed to alloc buffer stream='%s' utc_start=%llu utc_end=%llu
```

## Parameters

- Argument 1 — Stream: Text naming the application and stream.
- Argument 2 — UTC start: 64-bit requested UTC start in seconds since the Unix epoch; zero leaves this bound unspecified.
- Argument 3 — UTC end: 64-bit requested UTC end in seconds since the Unix epoch; zero leaves this bound unspecified.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to alloc buffer stream='live/camera' utc_start=1700000000 utc_end=1700000060
```

## Short description

DVR export buffer allocation failed.

## Trigger condition

The MP4 or subtitle export cannot allocate its initial 1 MiB output buffer.

## Explanation

Export construction throws a fatal-session exception.

## Possible causes

- The MP4 or subtitle export cannot allocate its initial 1 MiB output buffer.

## What to check

1. Check process memory availability.

## Recommended action

Check process memory availability.
