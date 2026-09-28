# AVC initialization lacks a video track

Product: Nimble Streamer
Component: Fragmented media to transport stream packaging
Logging function: log_e
Log level: error
Documentation ID: log_e-42fcb2421f82bde7

## Log pattern

```text
cannot load video_init_segment(video trak doesn't exist)
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
cannot load video_init_segment(video trak doesn't exist)
```

## Short description

AVC initialization lacks a video track.

## Trigger condition

The parsed video initialization segment has no video track.

## Explanation

AVC transport-header construction fails and the requested output chunk cannot be produced.

## Possible causes

- The parsed video initialization segment has no video track.

## What to check

1. Check that the supplied initialization segment contains the video track matching the media fragment.

## Recommended action

Provide a valid matching AVC initialization segment.
