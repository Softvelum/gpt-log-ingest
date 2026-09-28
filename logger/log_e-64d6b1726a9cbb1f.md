# Icecast audio frame rejected by live stream

Product: Nimble Streamer
Component: Icecast ingest and metadata
Logging function: log_e
Log level: error
Documentation ID: log_e-64d6b1726a9cbb1f

## Log pattern

```text
icecast stream failed to process audio message %s
```

## Parameters

- Argument 1 — URL: Configured source URL; it can be empty for an incoming publisher.

## Log examples

Illustrative examples; these are not captured server logs.

```text
icecast stream failed to process audio message http://radio.example.net/live
```

## Short description

Icecast audio frame rejected by live stream.

## Trigger condition

A completed AAC payload or MP3 frame cannot be accepted by live-stream processing.

## Explanation

The session is shut down before the frame is appended to the Icecast output chunk.

## Possible causes

- A completed AAC payload or MP3 frame cannot be accepted by live-stream processing.

## What to check

1. Check preceding live-stream audio diagnostics and source audio continuity.
