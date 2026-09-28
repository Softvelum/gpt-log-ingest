# Video packet has an invalid synchronization state

Product: Nimble Streamer
Component: Transcoding output
Logging function: log_e
Log level: error
Documentation ID: log_e-fcf02f7dc4324f8d

## Log pattern

```text
failed to write video pkt, wrong sync_stream=%d state=%d for [%s/%s]
```

## Parameters

- Argument 1 — Synchronization mode: Integer mode: 0 leaves timestamps unchanged, 1 aligns video to audio, 2 aligns audio to video.
- Argument 2 — Processing state: Integer state: 0 initial, 1 video only, 2 audio only, 3 processing audio after video, 4 processing video with audio paused, 5 waiting for video, 6 waiting for audio, 7 finding first audio, 8 processing audio with video paused.
- Argument 3 — Application: Output application name, text.
- Argument 4 — Stream: Output stream name, text.

Without synchronization or with audio following video, video-only and the two video-led combined states are accepted. With video following audio, only the video-processing/audio-paused state is accepted.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to write video pkt, wrong sync_stream=2 state=0 for [live/encoded]
```

## Short description

Video packet has an invalid synchronization state.

## Trigger condition

Packet synchronization encounters an unrecognized mode or a processing state not accepted for video in that mode.

## Explanation

The packet is not delivered and the current output operation fails.

## Possible causes

- The worker lifecycle state and synchronization mode are inconsistent at packet delivery.

## What to check

1. Check the reported mode and state and recent source or scenario updates.

## Recommended action

Restart the output with consistent synchronization settings and investigate repeated state failures.
