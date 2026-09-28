# DVR DRM waiting queue is full

Product: Nimble Streamer
Component: DVR playback conversion
Logging function: log_e
Log level: error
Documentation ID: log_e-c2ea2d6fecbe0204

## Log pattern

```text
dropped DVR transmuxer request for key='%s' as queue size limit=%u reached
```

## Parameters

- Argument 1 — Cache key: Text identifying the requested cache output.
- Argument 2 — Queue limit: Unsigned maximum number of postponed requests; currently 2000.

## Log examples

Illustrative examples; these are not captured server logs.

```text
dropped DVR transmuxer request for key='/live/camera/dvr_v_p1_90000.fmp4' as queue size limit=2000 reached
```

## Short description

DVR DRM waiting queue is full.

## Trigger condition

A segment or HLS/DASH playlist requires keys while 2000 requests are already postponed.

## Explanation

The cache item is canceled and this request is dropped.

## Possible causes

- A segment or HLS/DASH playlist requires keys while 2000 requests are already postponed.

## What to check

1. Check DRM key-server response failures and the accumulation of requests awaiting keys.

## Recommended action

Check DRM key-server response failures and the accumulation of requests awaiting keys.
