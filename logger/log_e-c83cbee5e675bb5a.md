# HEVC initialization lacks a video track

Product: Nimble Streamer
Component: Fragmented media to transport stream packaging
Logging function: log_e
Log level: error
Documentation ID: log_e-c83cbee5e675bb5a

## Log pattern

```text
cannot load video_hevc_init_segment(video trak doesn't exist)
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
cannot load video_hevc_init_segment(video trak doesn't exist)
```

## Short description

HEVC initialization lacks a video track.

## Trigger condition

The parsed HEVC initialization segment has no video track.

## Explanation

HEVC transport-header construction returns failure.

## Possible causes

- The parsed HEVC initialization segment has no video track.

## What to check

1. Check the initialization segment video track and its correspondence to the fragment.

## Recommended action

Provide a matching HEVC initialization segment.
