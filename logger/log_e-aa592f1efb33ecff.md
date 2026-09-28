# HLS skips unready advertising content

Product: Nimble Streamer
Component: HLS and DASH packaging
Logging function: log_e
Log level: error
Documentation ID: log_e-aa592f1efb33ecff

## Log pattern

```text
app=%s stream=%s stream_name=%s session=%lu skip advertising content error %s
```

## Parameters

- Argument 1 — Application: Application associated with the advertising playback session.
- Argument 2 — Stream: Stream associated with the advertising playback session.
- Argument 3 — Rendition: Specific stream name being processed for this session.
- Argument 4 — Session: Numeric playback session identifier.
- Argument 5 — Content URI: URI of the unavailable advertising media.

## Log examples

Illustrative examples; these are not captured server logs.

```text
app=live stream=camera stream_name=live/camera session=123 skip advertising content error https://media.example/ad.mp4
```

## Short description

HLS skips unready advertising content.

## Trigger condition

Selected content is not ready, its remote wait has been used or is unavailable, and its error policy is skip.

## Explanation

The advertising state returns to selecting content and the playlist update continues.

## Possible causes

- Selected content is not ready, its remote wait has been used or is unavailable, and its error policy is skip.

## What to check

1. Check content load errors, wait settings, and the skip policy.

## Recommended action

Make the media ready before insertion if the advertisement must play.
