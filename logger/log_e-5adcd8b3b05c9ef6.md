# SRT publisher authorization failed

Product: Nimble Streamer
Component: SRT connections and session reporting
Logging function: log_e
Log level: error
Documentation ID: log_e-5adcd8b3b05c9ef6

## Log pattern

```text
url auth failed, streamid=[%s],ip=[%s],client=%d
```

## Parameters

- Argument 1 — Stream ID: Requested SRT stream identifier.
- Argument 2 — Peer: Publisher IP address.
- Argument 3 — Socket: SRT socket identifier.

## Log examples

Illustrative examples; these are not captured server logs.

```text
url auth failed, streamid=[live/news?srtauth=example],ip=[192.0.2.20],client=100
```

## Short description

SRT publisher authorization failed.

## Trigger condition

The accepted stream-ID publisher fails publish-signature processing or the applicable SRT/application credential check.

## Explanation

The incoming stream is not installed as an active camera.

## Possible causes

- The accepted stream-ID publisher fails publish-signature processing or the applicable SRT/application credential check.

## What to check

1. Check the requested application/stream and applicable publishing credentials.
