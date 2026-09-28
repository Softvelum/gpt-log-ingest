# DVR export silence send failed

Product: Nimble Streamer
Component: DVR export
Logging function: log_e
Log level: error
Documentation ID: log_e-a58326fd58ed7db5

## Log pattern

```text
failed to send silence sample, period=%d stream=%s utc_start=%lld utc_end=%lld
```

## Parameters

- Argument 1 — Recording period: Integer identifier of the continuous recording period.
- Argument 2 — Stream: Text naming the application and stream.
- Argument 3 — UTC start: 64-bit requested UTC start in seconds since the Unix epoch; zero leaves this bound unspecified.
- Argument 4 — UTC end: 64-bit requested UTC end in seconds since the Unix epoch; zero leaves this bound unspecified.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to send silence sample, period=1 stream=live/camera utc_start=1700000000 utc_end=1700000060
```

## Short description

DVR export silence send failed.

## Trigger condition

Sending a buffered silence chunk for audio/video alignment fails.

## Explanation

The export closes.

## Possible causes

- Sending a buffered silence chunk for audio/video alignment fails.

## What to check

1. Check the client connection and socket or TLS errors.

## Recommended action

Check the client connection and socket or TLS errors.
