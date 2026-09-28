# Video component added

Product: Nimble Streamer
Component: Transcoding output management
Logging function: log_d
Log level: debug
Documentation ID: log_d-a083ba812da0af7e

## Log pattern

```text
video encoder added for %s/%s
```

## Parameters

- Argument 1 — Application: Output application name, text.
- Argument 2 — Stream: Output stream name, text.

Initial output becomes video-only unless video must follow audio, when it waits for audio. Adding to audio-only output starts combined processing; with video following audio it first seeks an audio timestamp.

## Log examples

Illustrative examples; these are not captured server logs.

```text
video encoder added for live/encoded
```

## Short description

Video component added.

## Trigger condition

A video-settings update is applied in an initial or audio-only state where video can be added.

## Explanation

The worker changes its processing state and then configures the video source or encoder. This precedes successful encoder creation.
