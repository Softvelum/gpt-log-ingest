# AJA circulation state captured after a failed transfer

Product: Nimble Streamer
Component: AJA capture and playback
Logging function: log_e
Log level: error
Documentation ID: log_e-d4ab02d6d7dbcdda

## Log pattern

```text
AutoCirculate status device=%s/%d state=%d frames=%u buffer=%u available=%u good_transfers=%u started=%d
```

## Parameters

- Argument 1 — Device: Configured AJA device selector or display name, as indicated by the message.
- Argument 2 — Channel: Numeric AJA channel identifier used by the device API; it is not converted to a one-based display number.
- Argument 3 — State: Integer circulation-engine state reported by the AJA runtime.
- Argument 4 — Frame count: Number of device frames in the configured circulation range.
- Argument 5 — Buffer level: Device-reported buffered frame count.
- Argument 6 — Available frames: Number of currently available output frame slots.
- Argument 7 — Successful transfers: Count of successful transfers completed by this sender.
- Argument 8 — Started flag: One if playback circulation had started, otherwise zero.

## Log examples

Illustrative examples; these are not captured server logs.

```text
AutoCirculate status device=0/0 state=0 frames=7 buffer=0 available=7 good_transfers=0 started=0
```

## Short description

AJA circulation state captured after a failed transfer.

## Trigger condition

An output frame transfer fails and the subsequent device-status query succeeds.

## Explanation

This diagnostic records device state and queue information immediately before transfer processing returns failure. It is emitted only as supporting detail for the failed transfer.

## Possible causes

- An output frame transfer fails and the subsequent device-status query succeeds.

## What to check

1. Compare available output slots and buffered frames with whether playback had started.

## Related log messages

- [AJA frame transfer to the output device failed](log_e-90857504bd75f019.md)
