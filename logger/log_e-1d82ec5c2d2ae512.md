# HEVC live codec initialization failed

Product: Nimble Streamer
Component: MPEG-TS video conversion
Logging function: log_e
Log level: error
Documentation ID: log_e-1d82ec5c2d2ae512

## Log pattern

```text
MPEG2TSLiveStream::processHEVCVideoMessageInternal, set_video_codec_hevc failed
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
MPEG2TSLiveStream::processHEVCVideoMessageInternal, set_video_codec_hevc failed
```

## Short description

HEVC live codec initialization failed.

## Trigger condition

One or more HEVC parameter sets changed, all three stored parameter-set groups are nonempty, and installing their codec configuration returns false.

## Explanation

The converter returns false before delivering the frame. Direct input and pending-frame flush reset video state after this failure is propagated. This message does not identify which parameter set was rejected.

## Possible causes

- The live stream could not accept the collected HEVC VPS, SPS, and PPS configuration.

## What to check

1. Check prior HEVC configuration diagnostics and parameter-set consistency.

## Recommended action

Correct the indicated HEVC parameter-set problem.
