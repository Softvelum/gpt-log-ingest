# DVR MP3 frame muting failed

Product: Nimble Streamer
Component: DVR archive maintenance
Logging function: log_e
Log level: error
Documentation ID: log_e-96d18f84de0f01af

## Log pattern

```text
failed to mute mp3 frames, segment utc_time=%lld start_time=%lld duration=%lld period=%d
```

## Parameters

- Argument 1 — UTC time: 64-bit segment UTC time in milliseconds since the Unix epoch.
- Argument 2 — Media start: 64-bit media start timestamp in the segment track timescale.
- Argument 3 — Media duration: 64-bit duration in the segment track timescale.
- Argument 4 — Period: Unsigned recording period identifier.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to mute mp3 frames, segment utc_time=1700000000000 start_time=90000 duration=540000 period=1
```

## Short description

DVR MP3 frame muting failed.

## Trigger condition

Replacing an MP3 sample with silence returns failure or an empty sample buffer.

## Explanation

The segment mute operation returns failure before writing replacement data.

## Possible causes

- Replacing an MP3 sample with silence returns failure or an empty sample buffer.

## What to check

1. Check MP3 frame validity and the supported MP3 format.

## Recommended action

Check MP3 frame validity and the supported MP3 format.
