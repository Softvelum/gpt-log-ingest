# Playlist AVC packet conversion failed

Product: Nimble Streamer
Component: Server playlists
Logging function: log_e
Log level: error
Documentation ID: log_e-ebd76b4722dfeb64

## Log pattern

```text
failed to conv avc annexb to avcc, file=%s
```

## Parameters

- Argument 1 — Content location: Text identifying a local media file or a remote cache key.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to conv avc annexb to avcc, file=/media/program.mp4
```

## Short description

Playlist AVC packet conversion failed.

## Trigger condition

Converting an MPEG-TS H.264 packet from start-code framing to length-prefixed framing fails.

## Explanation

Reading the next media packet returns failure.

## Possible causes

- Converting an MPEG-TS H.264 packet from start-code framing to length-prefixed framing fails.

## What to check

1. Check the H.264 packet structure in the source transport stream.

## Recommended action

Check the H.264 packet structure in the source transport stream.
