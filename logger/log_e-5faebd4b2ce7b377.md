# MP3 variable bitrate mode was rejected

Product: Nimble Streamer
Component: MP3 audio encoding
Logging function: log_e
Log level: error
Documentation ID: log_e-5faebd4b2ce7b377

## Log pattern

```text
failed to set vbr_mode=%d, status=%d
```

## Parameters

- Argument 1 — Mode: Numeric LAME variable-bitrate mode, integer.
- Argument 2 — Status: LAME return status, signed integer.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to set vbr_mode=3, status=-1
```

## Short description

MP3 variable bitrate mode was rejected.

## Trigger condition

In a LAME-enabled build, applying variable bitrate mode returns a nonzero status.

## Explanation

Encoder initialization stops and the audio-frame write reports failure.

## Possible causes

- LAME rejected variable bitrate mode.

## What to check

1. Check the selected variable-bitrate mode and the returned LAME status.

## Recommended action

Correct the rejected MP3 encoder setting.
