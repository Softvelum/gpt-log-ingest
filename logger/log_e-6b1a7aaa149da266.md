# MP3 frame size is zero

Product: Nimble Streamer
Component: Icecast ingest and metadata
Logging function: log_e
Log level: error
Documentation ID: log_e-6b1a7aaa149da266

## Log pattern

```text
icecast stream(mp3) frame size invalid %s
```

## Parameters

- Argument 1 — URL: Configured source URL; it can be empty for an incoming publisher.

## Log examples

Illustrative examples; these are not captured server logs.

```text
icecast stream(mp3) frame size invalid http://radio.example.net/live
```

## Short description

MP3 frame size is zero.

## Trigger condition

The frame size calculated from MPEG audio version, layer, bitrate, sample rate, and padding equals zero.

## Explanation

The ingest session is shut down.

## Possible causes

- The frame size calculated from MPEG audio version, layer, bitrate, sample rate, and padding equals zero.

## What to check

1. Check the source MPEG audio frame header and bitrate information.
