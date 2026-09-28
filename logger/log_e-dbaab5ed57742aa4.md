# HLS advertising cannot obtain a trailing live-chunk remainder

Product: Nimble Streamer
Component: HLS and DASH packaging
Logging function: log_e
Log level: error
Documentation ID: log_e-dbaab5ed57742aa4

## Log pattern

```text
app=%s stream=%s stream_name=%s session=%lu failed to get chunk remainder
```

## Parameters

- Argument 1 — Application: Application associated with the advertising playback session.
- Argument 2 — Stream: Stream associated with the advertising playback session.
- Argument 3 — Rendition: Specific stream name being processed for this session.
- Argument 4 — Session: Numeric playback session identifier.

## Log examples

Illustrative examples; these are not captured server logs.

```text
app=live stream=camera stream_name=live/camera session=123 failed to get chunk remainder
```

## Short description

HLS advertising cannot obtain a trailing live-chunk remainder.

## Trigger condition

The asynchronous trailing-remainder result is missing or reports failure.

## Explanation

The advertising playlist update fails. A successful but empty remainder uses a separate skip path and does not cause this log.

## Possible causes

- The asynchronous trailing-remainder result is missing or reports failure.

## What to check

1. Check the trailing-remainder media processing and source chunk availability.

## Recommended action

Correct the failed remainder processing before retrying the playlist.
