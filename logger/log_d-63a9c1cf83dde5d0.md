# Transcription audio chunk sent

Product: Nimble Streamer
Component: Live speech transcription
Logging function: log_d
Log level: debug
Documentation ID: log_d-63a9c1cf83dde5d0

## Log pattern

```text
sent audio chunk #%d start=%ld duration=%ld for stream='%s'
```

## Parameters

- Argument 1 — Sequence: Audio chunk sequence number.
- Argument 2 — Start: Chunk start timestamp in milliseconds.
- Argument 3 — Duration: Chunk duration in milliseconds.
- Argument 4 — Stream: Configured stream name.

## Log examples

Illustrative examples; these are not captured server logs.

```text
sent audio chunk #1 start=1000 duration=6000 for stream='live/news'
```

## Short description

Transcription audio chunk sent.

## Trigger condition

A complete binary audio chunk has been written to the websocket.

## Explanation

The chunk is removed from the local queue, its sequence advances, and the inactivity counter resets. Speechmatics waits for the audio acknowledgment; KWIKmotion can continue with subsequent chunks.

## Possible causes

- A complete binary audio chunk has been written to the websocket.

## What to check

1. Compare sequence and timing progression to confirm audio is being sent.
