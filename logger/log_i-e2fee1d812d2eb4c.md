# FFmpeg NVIDIA decoder GPU selected

Product: Nimble Streamer
Component: NVIDIA device contexts
Logging function: log_i
Log level: info
Documentation ID: log_i-e2fee1d812d2eb4c

## Log pattern

```text
assigned ffmpeg-nvenc decoder to gpu=%s, stream='%s'
```

## Parameters

- Argument 1 — GPU: CUDA device index passed to FFmpeg as text.
- Argument 2 — Stream: Input stream name, text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
assigned ffmpeg-nvenc decoder to gpu=0, stream='live/camera'
```

## Short description

FFmpeg NVIDIA decoder GPU selected.

## Trigger condition

The FFmpeg NVIDIA decoder GPU setting is auto.

## Explanation

The selected GPU will be used to get a shared FFmpeg CUDA device context. Selection minimizes the larger of encoder and decoder utilization with a temporary penalty for recent assignments. GPU zero is used if automatic balancing is unavailable or a selection query fails.
