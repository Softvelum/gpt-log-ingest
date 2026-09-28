# DVR export audio track cannot be created

Product: Nimble Streamer
Component: DVR export
Logging function: log_e
Log level: error
Documentation ID: log_e-f7ae50767ad0eb37

## Log pattern

```text
failed to add audio trak, stream=%s utc_start=%lld utc_end=%lld
```

## Parameters

- Argument 1 — Stream: Text naming the application and stream.
- Argument 2 — UTC start: 64-bit requested UTC start in seconds since the Unix epoch; zero leaves this bound unspecified.
- Argument 3 — UTC end: 64-bit requested UTC end in seconds since the Unix epoch; zero leaves this bound unspecified.

The audio-only occurrence has a different guard and does not reliably detect a missing audio track.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to add audio trak, stream=live/camera utc_start=1700000000 utc_end=1700000060
```

## Short description

DVR export audio track cannot be created.

## Trigger condition

Mixed export fails to add an audio track. The audio-only branch contains the same message but checks the already-created movie builder instead of the new track.

## Explanation

The guarded branch requests HTTP 500.

## Possible causes

- Mixed export fails to add an audio track. The audio-only branch contains the same message but checks the already-created movie builder instead of the new track.

## What to check

1. Check recorded audio initialization and resource errors.

## Recommended action

Check recorded audio initialization and resource errors.
