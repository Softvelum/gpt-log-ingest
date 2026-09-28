# DVR common encryption failed

Product: Nimble Streamer
Component: DVR playback conversion
Logging function: log_e
Log level: error
Documentation ID: log_e-60733ba31aaf2c7d

## Log pattern

```text
ProcessDvrStreamRequest failed to encrypt segment='%s' for [%s/%s]
```

## Parameters

- Argument 1 — Chunk name: Text naming the requested recording resource.
- Argument 2 — Application: Text naming the application.
- Argument 3 — Stream: Text naming the stream.

## Log examples

Illustrative examples; these are not captured server logs.

```text
ProcessDvrStreamRequest failed to encrypt segment='dvr_v_p1_90000.fmp4' for [live/camera]
```

## Short description

DVR common encryption failed.

## Trigger condition

Recorded segment encryption returns no output buffer.

## Explanation

The playback cache item is canceled.

## Possible causes

- Recorded segment encryption returns no output buffer.

## What to check

1. Check preceding DRM license, key-availability, initialization, and segment-encryption errors.

## Recommended action

Check preceding DRM license, key-availability, initialization, and segment-encryption errors.
