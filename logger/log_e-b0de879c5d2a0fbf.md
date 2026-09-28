# MP3 packet has no queued timestamp

Product: Nimble Streamer
Component: MP3 audio encoding
Logging function: log_e
Log level: error
Documentation ID: log_e-b0de879c5d2a0fbf

## Log pattern

```text
failed to get pts: pts queue is empty
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to get pts: pts queue is empty
```

## Short description

MP3 packet has no queued timestamp.

## Trigger condition

A complete encoded packet is buffered but the input timestamp queue is empty.

## Explanation

The packet write fails because no presentation timestamp can be assigned.

## Possible causes

- Encoded output has no corresponding retained input timestamp.

## What to check

1. Check encoder output/input frame accounting and preceding failures.

## Recommended action

Resolve the indicated MP3 encoding or frame-accounting failure.
