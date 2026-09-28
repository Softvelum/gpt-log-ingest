# VP8 decoder resets after a size change

Product: Nimble Streamer
Component: Video decoding
Logging function: log_d
Log level: debug
Documentation ID: log_d-ebea31d3f61b39ee

## Log pattern

```text
reset decoder on vp8 params change for [%s/%s]
```

## Parameters

- Argument 1 — Application: Application name, text.
- Argument 2 — Stream: Stream name, text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
reset decoder on vp8 params change for [live/camera]
```

## Short description

VP8 decoder resets after a size change.

## Trigger condition

Dynamic parameter changes are enabled, the decoder requires replacement on a change, and a keyframe reports different dimensions. Previous width and height must also be nonzero.

## Explanation

The old decoder is flushed and replaced, and output links are refreshed. The current keyframe is then decoded.
