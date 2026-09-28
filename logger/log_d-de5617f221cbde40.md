# DVR MP4 export closed

Product: Nimble Streamer
Component: DVR export
Logging function: log_d
Log level: debug
Documentation ID: log_d-de5617f221cbde40

## Log pattern

```text
shutdown: state=%d stream=%s utc_start=%lld utc_end=%lld
```

## Parameters

- Argument 1 — Previous state: Integer export state before shutdown.
- Argument 2 — Stream: Text naming the application and stream.
- Argument 3 — UTC start: 64-bit requested UTC start in seconds since the Unix epoch; zero leaves this bound unspecified.
- Argument 4 — UTC end: 64-bit requested UTC end in seconds since the Unix epoch; zero leaves this bound unspecified.

## Log examples

Illustrative examples; these are not captured server logs.

```text
shutdown: state=4 stream=live/camera utc_start=1700000000 utc_end=1700000060
```

## Short description

DVR MP4 export closed.

## Trigger condition

Export finishes or a preparation, parsing, or transport error requests shutdown.

## Explanation

The peer socket is shut down and export state changes to finished.

## Possible causes

- Export finishes or a preparation, parsing, or transport error requests shutdown.

## What to check

1. Check preceding export errors if the download was incomplete.

## Recommended action

Check preceding export errors if the download was incomplete.
