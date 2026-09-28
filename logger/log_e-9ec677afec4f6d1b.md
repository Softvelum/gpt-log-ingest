# DVR export video segment disappeared

Product: Nimble Streamer
Component: DVR export
Logging function: log_e
Log level: error
Documentation ID: log_e-9ec677afec4f6d1b

## Log pattern

```text
failed to get video segment period=%d start_time=%lld stream=%s utc_start=%lld utc_end=%lld
```

## Parameters

- Argument 1 — Recording period: Integer identifier of the continuous recording period.
- Argument 2 — Segment start: 64-bit segment start timestamp in that track's media timescale.
- Argument 3 — Stream: Text naming the application and stream.
- Argument 4 — UTC start: 64-bit requested UTC start in seconds since the Unix epoch; zero leaves this bound unspecified.
- Argument 5 — UTC end: 64-bit requested UTC end in seconds since the Unix epoch; zero leaves this bound unspecified.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to get video segment period=1 start_time=90000 stream=live/camera utc_start=1700000000 utc_end=1700000060
```

## Short description

DVR export video segment disappeared.

## Trigger condition

A video segment selected earlier cannot be retrieved from the archive.

## Explanation

During preparation this fails and closes the export; during transmission the missing segment is skipped. Some transmission paths log the same failure twice.

## Possible causes

- A video segment selected earlier cannot be retrieved from the archive.

## What to check

1. Check concurrent archive cleanup and availability of the selected recording range.

## Recommended action

Check concurrent archive cleanup and availability of the selected recording range.
