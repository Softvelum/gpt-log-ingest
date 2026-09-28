# DVR HLS encryption initialization is unavailable

Product: Nimble Streamer
Component: DVR playback conversion
Logging function: log_e
Log level: error
Documentation ID: log_e-2c1bdf1e523f869b

## Log pattern

```text
failed to get init segment info app_stream='%s' name='%s'
```

## Parameters

- Argument 1 — Stream: Text naming the application and stream.
- Argument 2 — Initialization name: Text naming the required video initialization resource.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to get init segment info app_stream='live/camera' name='dvr_iv_p1.fmp4'
```

## Short description

DVR HLS encryption initialization is unavailable.

## Trigger condition

Encrypting a recorded HLS fragmented-MP4 video segment cannot locate its period initialization segment.

## Explanation

The cache item is canceled.

## Possible causes

- Encrypting a recorded HLS fragmented-MP4 video segment cannot locate its period initialization segment.

## What to check

1. Check the recorded period initialization data and concurrent archive cleanup.

## Recommended action

Check the recorded period initialization data and concurrent archive cleanup.
