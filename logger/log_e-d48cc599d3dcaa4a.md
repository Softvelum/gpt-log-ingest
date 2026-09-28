# Video encoder input dimensions changed

Product: Nimble Streamer
Component: Video encoder input stability
Logging function: log_e
Log level: error
Documentation ID: log_e-d48cc599d3dcaa4a

## Log pattern

```text
failed to change frame size from %dx%d to %dx%d on the fly for [%s]
```

## Parameters

- Argument 1 — Old width: Old width, integer pixels.
- Argument 2 — Old height: Old height, integer pixels.
- Argument 3 — New width: New width, integer pixels.
- Argument 4 — New height: New height, integer pixels.
- Argument 5 — Stream: Output stream name, text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to change frame size from 1920x1080 to 1280x720 on the fly for [live/camera]
```

## Short description

Video encoder input dimensions changed.

## Trigger condition

An initialized FFmpeg or native Quick Sync encoder receives dimensions different from its initial frame dimensions.

## Explanation

The frame write returns failure; this session cannot change size dynamically.

## Possible causes

- The source or video filter changes resolution during encoding.

## What to check

1. Compare the original and incoming dimensions and upstream format changes.

## Recommended action

Keep the encoder dimensions stable or restart encoding for the new resolution.
