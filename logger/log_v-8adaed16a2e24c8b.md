# DVR conversion timing checkpoint

Product: Nimble Streamer
Component: DVR playback conversion
Logging function: log_v
Log level: verbose
Documentation ID: log_v-8adaed16a2e24c8b

## Log pattern

```text
[DTT::PR] stream='%s' chunk='%s' key='%s' t=%lld (%lld) %s
```

## Parameters

- Argument 1 — Stream: Text naming the application and stream.
- Argument 2 — Chunk name: Text naming the requested recording resource.
- Argument 3 — Cache key: Text identifying the requested cache output.
- Argument 4 — Current time: 64-bit wall-clock timestamp in microseconds since the Unix epoch.
- Argument 5 — Elapsed time: 64-bit elapsed microseconds since processing started.
- Argument 6 — Checkpoint: Text: 1 after HLS playlist generation, 2 after segment selection, 3 before combined MPEG-TS generation, or fin at completion.

## Log examples

Illustrative examples; these are not captured server logs.

```text
[DTT::PR] stream='live/camera' chunk='dvr_v_p1_90000.fmp4' key='/live/camera/dvr_v_p1_90000.fmp4' t=1700000000002500 (2500) fin
```

## Short description

DVR conversion timing checkpoint.

## Trigger condition

Verbose timing is recorded after a processing checkpoint or when the request attempt ends.

## Explanation

Elapsed time includes only this attempt; fin is emitted even when the request returns early or fails.

## Possible causes

- Verbose timing is recorded after a processing checkpoint or when the request attempt ends.

## What to check

1. Use the elapsed time and checkpoint to locate slow playback conversion.

## Recommended action

Use the elapsed time and checkpoint to locate slow playback conversion.
