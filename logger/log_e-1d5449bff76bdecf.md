# DVR export video initialization is unavailable

Product: Nimble Streamer
Component: DVR export
Logging function: log_e
Log level: error
Documentation ID: log_e-1d5449bff76bdecf

## Log pattern

```text
failed to get video codec info period=%d stream=%s utc_start=%lld utc_end=%lld
```

## Parameters

- Argument 1 — Recording period: Integer identifier of the continuous recording period.
- Argument 2 — Stream: Text naming the application and stream.
- Argument 3 — UTC start: 64-bit requested UTC start in seconds since the Unix epoch; zero leaves this bound unspecified.
- Argument 4 — UTC end: 64-bit requested UTC end in seconds since the Unix epoch; zero leaves this bound unspecified.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to get video codec info period=1 stream=live/camera utc_start=1700000000 utc_end=1700000060
```

## Short description

DVR export video initialization is unavailable.

## Trigger condition

At a new recording period, the video initialization segment cannot be found or mapped.

## Explanation

Video parsing fails and the export closes.

## Possible causes

- At a new recording period, the video initialization segment cannot be found or mapped.

## What to check

1. Check the period's recorded video initialization segment.

## Recommended action

Check the period's recorded video initialization segment.
