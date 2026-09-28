# HLS packed MP3 segment ends inside a frame

Product: Nimble Streamer
Component: HLS and MPEG-TS HTTP input
Logging function: log_e
Log level: error
Documentation ID: log_e-158a3ea44f2bbf86

## Log pattern

```text
failed to process incomplete hls packed audio mp3 chunk, url=%s
```

## Parameters

- Argument 1 — url: Current origin URL.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to process incomplete hls packed audio mp3 chunk, url=https://origin.example/audio.mp3
```

## Short description

HLS packed MP3 segment ends inside a frame.

## Trigger condition

An MP3 frame header is recognized, but the remaining segment bytes are fewer than that frame length.

## Explanation

The frame loop stops. Previously collected complete frames are still passed onward, and this branch itself does not shut down the input. The incomplete remainder is not processed as a frame.

## Possible causes

- The packed audio body ends before the declared MP3 frame is complete.

## What to check

1. Check the downloaded audio segment for truncation or invalid frame lengths.

## Recommended action

Provide complete MP3 frames in the origin segment.
