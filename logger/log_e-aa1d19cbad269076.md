# DVR combined video transmission raised an exception

Product: Nimble Streamer
Component: DVR export
Logging function: log_e
Log level: error
Documentation ID: log_e-aa1d19cbad269076

## Log pattern

```text
unknown exception caught while send mixed video segment, stream=%s utc_start=%lld utc_end=%lld
```

## Parameters

- Argument 1 — Stream: Text naming the application and stream.
- Argument 2 — UTC start: 64-bit requested UTC start in seconds since the Unix epoch; zero leaves this bound unspecified.
- Argument 3 — UTC end: 64-bit requested UTC end in seconds since the Unix epoch; zero leaves this bound unspecified.

## Log examples

Illustrative examples; these are not captured server logs.

```text
unknown exception caught while send mixed video segment, stream=live/camera utc_start=1700000000 utc_end=1700000060
```

## Short description

DVR combined video transmission raised an exception.

## Trigger condition

Preparing or sending the next video segment raises an otherwise unclassified exception.

## Explanation

The export closes.

## Possible causes

- Preparing or sending the next video segment raises an otherwise unclassified exception.

## What to check

1. Check the affected recording fragment and preceding parsing or transport errors.

## Recommended action

Check the affected recording fragment and preceding parsing or transport errors.
