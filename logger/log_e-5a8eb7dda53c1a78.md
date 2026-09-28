# MP3 bit reservoir setting was rejected

Product: Nimble Streamer
Component: MP3 audio encoding
Logging function: log_e
Log level: error
Documentation ID: log_e-5a8eb7dda53c1a78

## Log pattern

```text
failed to set reservoir=%d, status=%d
```

## Parameters

- Argument 1 — Enabled: Requested bit-reservoir enable value, 0 or 1; the inverse is sent as the disable flag.
- Argument 2 — Status: LAME return status, signed integer.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to set reservoir=0, status=-1
```

## Short description

MP3 bit reservoir setting was rejected.

## Trigger condition

In a LAME-enabled build, applying bit reservoir setting returns a nonzero status.

## Explanation

Encoder initialization stops and the audio-frame write reports failure.

## Possible causes

- LAME rejected bit reservoir setting.

## What to check

1. Check the bit-reservoir setting and the returned LAME status.

## Recommended action

Correct the rejected MP3 encoder setting.
