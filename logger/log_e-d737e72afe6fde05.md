# MP3 encoding quality was rejected

Product: Nimble Streamer
Component: MP3 audio encoding
Logging function: log_e
Log level: error
Documentation ID: log_e-d737e72afe6fde05

## Log pattern

```text
failed to set quality=%d, status=%d
```

## Parameters

- Argument 1 — Quality: LAME quality setting, integer from 0 through 9; zero is best quality.
- Argument 2 — Status: LAME return status, signed integer.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to set quality=2, status=-1
```

## Short description

MP3 encoding quality was rejected.

## Trigger condition

In a LAME-enabled build, applying encoding quality returns a nonzero status.

## Explanation

Encoder initialization stops and the audio-frame write reports failure.

## Possible causes

- LAME rejected encoding quality.

## What to check

1. Check the quality setting and the returned LAME status.

## Recommended action

Correct the rejected MP3 encoder setting.
