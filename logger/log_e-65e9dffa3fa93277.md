# DVR export final padding header send failed

Product: Nimble Streamer
Component: DVR export
Logging function: log_e
Log level: error
Documentation ID: log_e-65e9dffa3fa93277

## Log pattern

```text
failed to send free size=%d stream=%s utc_start=%lld utc_end=%lld
```

## Parameters

- Argument 1 — Padding box bytes: Total free-box size in bytes, including its header.
- Argument 2 — Stream: Text naming the application and stream.
- Argument 3 — UTC start: 64-bit requested UTC start in seconds since the Unix epoch; zero leaves this bound unspecified.
- Argument 4 — UTC end: 64-bit requested UTC end in seconds since the Unix epoch; zero leaves this bound unspecified.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to send free size=28 stream=live/camera utc_start=1700000000 utc_end=1700000060
```

## Short description

DVR export final padding header send failed.

## Trigger condition

Sending the free-space box header fails.

## Explanation

The export closes.

## Possible causes

- Sending the free-space box header fails.

## What to check

1. Check the client connection and transport errors.

## Recommended action

Check the client connection and transport errors.
