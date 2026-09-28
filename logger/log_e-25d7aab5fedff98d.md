# Video frames were dropped because processing lagged

Product: Nimble Streamer
Component: Transcoding output
Logging function: log_e
Log level: error
Documentation ID: log_e-25d7aab5fedff98d

## Log pattern

```text
encoder too slow, dropped %llu/%llu video frames for [%s/%s]
```

## Parameters

- Argument 1 — Dropped frames: Unsigned 64-bit number of frames discarded since the previous report.
- Argument 2 — Accounted frames: Unsigned 64-bit count of processed attempts plus expired or synchronization-discarded frames in the reporting interval; not a lifetime total.
- Argument 3 — Application: Output application name, text.
- Argument 4 — Stream: Output stream name, text.

Both frame counts are unsigned 64-bit values passed to unsigned long-long conversions. These types differ on platforms where the 64-bit typedef is unsigned long.

## Log examples

Both frame counts are unsigned 64-bit values passed to unsigned long-long conversions. These types differ on platforms where the 64-bit typedef is unsigned long. A reliable example is omitted because the original call does not ensure matching printf argument types.

## Short description

Video frames were dropped because processing lagged.

## Trigger condition

The worker has counted dropped video frames and this is the first drop report or its processed counter has reached 500.

## Explanation

The dropped and total-accounted counts are logged and reset; processing continues from the available frame.

## Possible causes

- The reader fell behind retained frames; video also deliberately skips frames as backlog rises. Synchronization may discard further backlog when this component follows the other.

## What to check

1. Compare drop counts with processing capacity, buffer size, and host or GPU load.

## Recommended action

Reduce encoding demand or improve processing capacity to prevent continued frame loss.
