# Video encoder resets after input property changes

Product: Nimble Streamer
Component: Transcoding output
Logging function: log_d
Log level: debug
Documentation ID: log_d-175ddd44fa9d548e

## Log pattern

```text
video encoder frame params changed from %dx%d/%d to %dx%d/%d for [%s/%s]
```

## Parameters

- Argument 1 — Old width: Previous encoder width, integer pixels.
- Argument 2 — Old height: Previous encoder height, integer pixels.
- Argument 3 — Old format: Previous FFmpeg pixel-format identifier, integer.
- Argument 4 — New width: Incoming frame width, integer pixels.
- Argument 5 — New height: Incoming frame height, integer pixels.
- Argument 6 — New format: Incoming FFmpeg pixel-format identifier, integer.
- Argument 7 — Application: Output application name, text.
- Argument 8 — Stream: Output stream name, text.

Shared by keyframe and ordinary-frame submission. Depending on the encoder, aspect ratio, color metadata, or hardware-frame context changes can also trigger replacement even when the printed dimensions and format match.

## Log examples

Illustrative examples; these are not captured server logs.

```text
video encoder frame params changed from 1280x720/0 to 1920x1080/0 for [live/encoded]
```

## Short description

Video encoder resets after input property changes.

## Trigger condition

Dynamic parameter changes are enabled and the selected encoder reports that the frame no longer matches its active parameters.

## Explanation

The current encoder is flushed and replaced before the frame is written. Replacement or writing may still fail.
