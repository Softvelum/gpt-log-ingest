# DVR audio index contains another segment type

Product: Nimble Streamer
Component: DVR archive maintenance
Logging function: log_e
Log level: error
Documentation ID: log_e-ebc4c8f3865bc517

## Log pattern

```text
[DvrA::RemAS] invalid segment type=%u file='%s'
```

## Parameters

- Argument 1 — Segment type: Integer recorded-segment category: 2 video, 3 audio, 6 DVB subtitle, 7 KLV, 8 WebVTT.
- Argument 2 — Storage file: Text naming the recorded media storage file.

## Log examples

Illustrative examples; these are not captured server logs.

```text
[DvrA::RemAS] invalid segment type=3 file='/data/dvr/live/camera/1700000000000.dat'
```

## Short description

DVR audio index contains another segment type.

## Trigger condition

The audio-removal path receives a segment whose type is not audio.

## Explanation

The removal operation returns failure without deleting this entry.

## Possible causes

- The audio-removal path receives a segment whose type is not audio.

## What to check

1. Check archive index consistency and preceding load or cleanup errors.

## Recommended action

Check archive index consistency and preceding load or cleanup errors.
