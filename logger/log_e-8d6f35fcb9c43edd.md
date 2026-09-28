# DVR video export preparation raised an exception

Product: Nimble Streamer
Component: DVR export
Logging function: log_e
Log level: error
Documentation ID: log_e-8d6f35fcb9c43edd

## Log pattern

```text
BaseMP4LoaderException caught while parse video segments stream=%s utc_start=%lld utc_end=%lld
```

## Parameters

- Argument 1 — Stream: Text naming the application and stream.
- Argument 2 — UTC start: 64-bit requested UTC start in seconds since the Unix epoch; zero leaves this bound unspecified.
- Argument 3 — UTC end: 64-bit requested UTC end in seconds since the Unix epoch; zero leaves this bound unspecified.

## Log examples

Illustrative examples; these are not captured server logs.

```text
BaseMP4LoaderException caught while parse video segments stream=live/camera utc_start=1700000000 utc_end=1700000060
```

## Short description

DVR video export preparation raised an exception.

## Trigger condition

Parsing recorded video fragments raises an MP4 parser exception.

## Explanation

Preparation returns failure and the export closes.

## Possible causes

- Parsing recorded video fragments raises an MP4 parser exception.

## What to check

1. Check the selected recording fragments and preceding parsing errors.

## Recommended action

Check the selected recording fragments and preceding parsing errors.
