# MP3 hard minimum bitrate was rejected

Product: Nimble Streamer
Component: MP3 audio encoding
Logging function: log_e
Log level: error
Documentation ID: log_e-e3da62ceaf6c1bff

## Log pattern

```text
failed to set vbr_hard_min=%d, status=%d
```

## Parameters

- Argument 1 — Enabled: Requested hard-minimum control value, positive integer.
- Argument 2 — Status: LAME return status, signed integer.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to set vbr_hard_min=1, status=-1
```

## Short description

MP3 hard minimum bitrate was rejected.

## Trigger condition

In a LAME-enabled build, applying hard minimum bitrate returns a nonzero status.

## Explanation

Encoder initialization stops and the audio-frame write reports failure.

## Possible causes

- LAME rejected hard minimum bitrate.

## What to check

1. Check the hard-minimum setting and the returned LAME status.

## Recommended action

Correct the rejected MP3 encoder setting.
