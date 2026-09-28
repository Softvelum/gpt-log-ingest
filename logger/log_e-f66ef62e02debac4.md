# Disabled transcoder license ends WebVTT synchronization

Product: Nimble Streamer
Component: Live media packaging and timed metadata
Logging function: log_e
Log level: error
Documentation ID: log_e-f66ef62e02debac4

## Log pattern

```text
transcoder license disabled. Stop WebVTT processing [%s/%s]
```

## Parameters

- Argument 1 — Application: Application containing the live stream.
- Argument 2 — Stream: Live stream name.

## Log examples

Illustrative examples; these are not captured server logs.

```text
transcoder license disabled. Stop WebVTT processing [live/camera]
```

## Short description

Disabled transcoder license ends WebVTT synchronization.

## Trigger condition

Audio or video reaches the WebVTT synchronization path while the transcoder license is disabled.

## Explanation

All queued media messages are released and the subtitle synchronization queue is removed. The call returns success.

## Possible causes

- Audio or video reaches the WebVTT synchronization path while the transcoder license is disabled.

## What to check

1. Check current transcoder license availability and preceding license messages.

## Recommended action

Restore a valid transcoder license if subtitle processing is required.
