# MP3 minimum variable bitrate was rejected

Product: Nimble Streamer
Component: MP3 audio encoding
Logging function: log_e
Log level: error
Documentation ID: log_e-a63d020e60c7d2cd

## Log pattern

```text
failed to set vbr_min_bitrate_kbps=%d, status=%d
```

## Parameters

- Argument 1 — Bitrate: Requested minimum bitrate in kilobits per second, integer.
- Argument 2 — Status: LAME return status, signed integer.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to set vbr_min_bitrate_kbps=64, status=-1
```

## Short description

MP3 minimum variable bitrate was rejected.

## Trigger condition

In a LAME-enabled build, applying minimum variable bitrate returns a nonzero status.

## Explanation

Encoder initialization stops and the audio-frame write reports failure.

## Possible causes

- LAME rejected minimum variable bitrate.

## What to check

1. Check the minimum bitrate and the returned LAME status.

## Recommended action

Correct the rejected MP3 encoder setting.
