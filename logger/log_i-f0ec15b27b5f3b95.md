# FFmpeg NVIDIA encoder GPU selected

Product: Nimble Streamer
Component: NVIDIA device contexts
Logging function: log_i
Log level: info
Documentation ID: log_i-f0ec15b27b5f3b95

## Log pattern

```text
assigned ffmpeg-nvenc encoder to gpu=%s, stream='%s'
```

## Parameters

- Argument 1 — GPU: CUDA device index passed to FFmpeg as text.
- Argument 2 — Stream: Output stream name, text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
assigned ffmpeg-nvenc encoder to gpu=0, stream='live/camera'
```

## Short description

FFmpeg NVIDIA encoder GPU selected.

## Trigger condition

The FFmpeg NVIDIA encoder GPU setting is auto.

## Explanation

The selected GPU will be used to get a shared FFmpeg CUDA device context. Selection minimizes encoder utilization with a temporary penalty for recent assignments. GPU zero is used if automatic balancing is unavailable or a selection query fails.
