# DASH resets after a protocol timestamp discontinuity

Product: Nimble Streamer
Component: Live stream codec and packaging setup
Logging function: log_i
Log level: info
Documentation ID: log_i-5fe425c8b7b6ec88

## Log pattern

```text
%s check dash availability(protocol) time reset for [%s/%s]
```

## Parameters

- Argument 1 — Media: video or audio track triggering the reset.
- Argument 2 — Application: Application containing the affected live stream.
- Argument 3 — Stream: Live stream name.

## Log examples

Illustrative examples; these are not captured server logs.

```text
video check dash availability(protocol) time reset for [live/camera]
```

## Short description

DASH resets after a protocol timestamp discontinuity.

## Trigger condition

An established DASH presentation sees its segment protocol-time-reset flag set.

## Explanation

The DASH segment list is reset, the current track timestamp is reinitialized, and availability/publish times become unset until a later segment. Enabled presentation-time offsets are also updated.

## Possible causes

- An established DASH presentation sees its segment protocol-time-reset flag set.

## What to check

1. Check source timestamp continuity for the indicated media track.

## Recommended action

Investigate repeated unexpected source timestamp changes.
