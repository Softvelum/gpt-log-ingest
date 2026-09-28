# DVR conversion request raised an exception

Product: Nimble Streamer
Component: DVR playback conversion
Logging function: log_e
Log level: error
Documentation ID: log_e-f56455b8ad5a0cb9

## Log pattern

```text
std::exception caught while processing request chunk_name='%s' cache_item_key='%s' what()='%s'
```

## Parameters

- Argument 1 — Chunk name: Text naming the requested recording resource.
- Argument 2 — Cache key: Text identifying the requested cache output.
- Argument 3 — Exception: Text describing the exception.

## Log examples

Illustrative examples; these are not captured server logs.

```text
std::exception caught while processing request chunk_name='dvr_v_p1_90000.fmp4' cache_item_key='/live/camera/dvr_v_p1_90000.fmp4' what()='allocation failed'
```

## Short description

DVR conversion request raised an exception.

## Trigger condition

Processing a queued playback conversion request throws a standard exception.

## Explanation

The worker logs the error and continues with later requests; this catch does not itself cancel the affected cache item.

## Possible causes

- Processing a queued playback conversion request throws a standard exception.

## What to check

1. Check the exception text and preceding media or memory errors.

## Recommended action

Check the exception text and preceding media or memory errors.
