# MP3 combined encoder parameters was rejected

Product: Nimble Streamer
Component: MP3 audio encoding
Logging function: log_e
Log level: error
Documentation ID: log_e-ded8a7d717a8a133

## Log pattern

```text
failed to set lame init params, status=%d
```

## Parameters

- Argument 1 — Status: LAME return status, signed integer.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to set lame init params, status=-1
```

## Short description

MP3 combined encoder parameters was rejected.

## Trigger condition

In a LAME-enabled build, applying combined encoder parameters returns a nonzero status.

## Explanation

Encoder initialization stops and the audio-frame write reports failure.

## Possible causes

- LAME rejected combined encoder parameters.

## What to check

1. Check the complete set of selected audio encoding parameters and the returned LAME status.

## Recommended action

Correct the rejected MP3 encoder setting.
