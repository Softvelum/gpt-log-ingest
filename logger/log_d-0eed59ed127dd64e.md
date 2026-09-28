# Audio component removed

Product: Nimble Streamer
Component: Transcoding output management
Logging function: log_d
Log level: debug
Documentation ID: log_d-0eed59ed127dd64e

## Log pattern

```text
audio encoder removed for %s/%s
```

## Parameters

- Argument 1 — Application: Output application name, text.
- Argument 2 — Stream: Output stream name, text.

Removal while waiting for video stops the audio-dependent worker. When video follows audio, combined and finding-first-audio states move to waiting for audio; otherwise combined output becomes video-only.

## Log examples

Illustrative examples; these are not captured server logs.

```text
audio encoder removed for live/encoded
```

## Short description

Audio component removed.

## Trigger condition

New output settings remove audio while the worker applies a supported state transition.

## Explanation

An audio-only worker stops. A combined worker releases audio and either continues video alone or waits for audio when video must follow audio.
