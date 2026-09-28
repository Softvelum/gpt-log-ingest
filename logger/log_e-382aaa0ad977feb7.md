# Encoder is still waiting for video

Product: Nimble Streamer
Component: Transcoding output
Logging function: log_e
Log level: error
Documentation ID: log_e-382aaa0ad977feb7

## Log pattern

```text
debugReadVideoFrameWaitForNewItems for [%s/%s]
```

## Parameters

- Argument 1 — Application: Output application name, text.
- Argument 2 — Stream: Output stream name, text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
debugReadVideoFrameWaitForNewItems for [live/encoded]
```

## Short description

Encoder is still waiting for video.

## Trigger condition

Encoder debug logging is enabled and consecutive wait-for-new-video observations exceed 1000.

## Explanation

The debug counter resets. The worker continues waiting; error severity here does not itself mean an encoding failure.
