# HLS discontinuity sequence counters are realigned

Product: Nimble Streamer
Component: HLS and DASH packaging
Logging function: log_d
Log level: debug
Documentation ID: log_d-15c6a4bcb17e0cb0

## Log pattern

```text
reset discontinuity sequences for [%s/%s]
```

## Parameters

- Argument 1 — Application: Application of the affected chunk list.
- Argument 2 — Stream: Stream of the affected chunk list.

## Log examples

Illustrative examples; these are not captured server logs.

```text
reset discontinuity sequences for [live/camera]
```

## Short description

HLS discontinuity sequence counters are realigned.

## Trigger condition

A companion media or subtitle list has advanced by more than its retained chunk window and has a larger discontinuity sequence than the current list.

## Explanation

The relevant current and companion lists reset their discontinuity sequence bases to restore alignment. Retained discontinuity counts are adjusted with those bases.

## Possible causes

- A companion media or subtitle list has advanced by more than its retained chunk window and has a larger discontinuity sequence than the current list.

## What to check

1. Compare audio, video, and subtitle playlist progress around periods when one track stopped producing chunks.

## Recommended action

Investigate repeated track stalls or timestamp discontinuities.
