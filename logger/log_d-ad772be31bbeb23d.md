# Video arrived for audio synchronization

Product: Nimble Streamer
Component: Transcoding output management
Logging function: log_d
Log level: debug
Documentation ID: log_d-ad772be31bbeb23d

## Log pattern

```text
video encoder added, sync audio to video for %s/%s
```

## Parameters

- Argument 1 — Application: Output application name, text.
- Argument 2 — Stream: Output stream name, text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
video encoder added, sync audio to video for live/encoded
```

## Short description

Video arrived for audio synchronization.

## Trigger condition

Audio-to-video synchronization is selected and video is added while the worker was waiting for video.

## Explanation

If a decoded-audio buffer exists its read position advances to the current write position; combined processing starts with audio paused until video establishes timing.
