# DVR export video track cannot be created

Product: Nimble Streamer
Component: DVR export
Logging function: log_e
Log level: error
Documentation ID: log_e-1eb2b052658ef2cc

## Log pattern

```text
failed to add video trak, stream=%s utc_start=%lld utc_end=%lld
```

## Parameters

- Argument 1 — Stream: Text naming the application and stream.
- Argument 2 — UTC start: 64-bit requested UTC start in seconds since the Unix epoch; zero leaves this bound unspecified.
- Argument 3 — UTC end: 64-bit requested UTC end in seconds since the Unix epoch; zero leaves this bound unspecified.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to add video trak, stream=live/camera utc_start=1700000000 utc_end=1700000060
```

## Short description

DVR export video track cannot be created.

## Trigger condition

The movie builder cannot add the video track.

## Explanation

An HTTP 500 response is requested.

## Possible causes

- The movie builder cannot add the video track.

## What to check

1. Check recorded video initialization data and resource failures.

## Recommended action

Check recorded video initialization data and resource failures.
