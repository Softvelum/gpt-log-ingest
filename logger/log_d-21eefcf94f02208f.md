# HLS advertising initializes its media sequence

Product: Nimble Streamer
Component: HLS and DASH packaging
Logging function: log_d
Log level: debug
Documentation ID: log_d-21eefcf94f02208f

## Log pattern

```text
app=%s stream=%s stream_name=%s session=%lu: initial sequence=%d
```

## Parameters

- Argument 1 — Application: Application associated with the advertising playback session.
- Argument 2 — Stream: Stream associated with the advertising playback session.
- Argument 3 — Rendition: Specific stream name being processed for this session.
- Argument 4 — Session: Numeric playback session identifier.
- Argument 5 — Sequence: Intended initial HLS media sequence; the 64-bit argument does not match the 32-bit conversion.

## Log examples

No reliable complete example is available because the sequence is a 64-bit value formatted with a 32-bit conversion.

## Short description

HLS advertising initializes its media sequence.

## Trigger condition

The first retained source chunk is selected while the advertising rendition is in its initialization phase.

## Explanation

Stream time, media sequence, discontinuity sequence, and current chunk state are initialized before SCTE-35 content processing. The sequence display is unreliable because a 64-bit value is passed to a 32-bit conversion.

## Possible causes

- The first retained source chunk is selected while the advertising rendition is in its initialization phase.

## What to check

1. Check the first source chunk and session playlist sequence.

## Recommended action

No action is required for expected initialization.
