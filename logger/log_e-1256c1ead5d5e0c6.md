# DVR conversion request raised an unclassified exception

Product: Nimble Streamer
Component: DVR playback conversion
Logging function: log_e
Log level: error
Documentation ID: log_e-1256c1ead5d5e0c6

## Log pattern

```text
unknown exception caught while processing request chunk_name='%s' cache_item_key='%s'
```

## Parameters

- Argument 1 — Chunk name: Text naming the requested recording resource.
- Argument 2 — Cache key: Text identifying the requested cache output.

## Log examples

Illustrative examples; these are not captured server logs.

```text
unknown exception caught while processing request chunk_name='dvr_v_p1_90000.fmp4' cache_item_key='/live/camera/dvr_v_p1_90000.fmp4'
```

## Short description

DVR conversion request raised an unclassified exception.

## Trigger condition

Processing a queued playback conversion request throws an exception outside the standard-exception handler.

## Explanation

The worker logs it and continues with later requests; this catch does not itself cancel the affected cache item.

## Possible causes

- Processing a queued playback conversion request throws an exception outside the standard-exception handler.

## What to check

1. Check preceding media and resource errors for the affected request.

## Recommended action

Check preceding media and resource errors for the affected request.
