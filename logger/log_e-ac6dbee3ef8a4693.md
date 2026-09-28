# Audio encoder bitrate was rejected

Product: Nimble Streamer
Component: MP3 audio encoding
Logging function: log_e
Log level: error
Documentation ID: log_e-ac6dbee3ef8a4693

## Log pattern

```text
failed to set bitrate=%d, status=%d
```

## Parameters

- Argument 1 — Bitrate: Requested bitrate in bits per second, integer, for both LAME and FDK occurrences.
- Argument 2 — Library status: Library-specific status code, integer; not an operating system error.

The status belongs to LAME or FDK according to the encoder. Their numeric error spaces differ.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to set bitrate=128000, status=1
```

## Short description

Audio encoder bitrate was rejected.

## Trigger condition

LAME or FDK AAC rejects its requested bitrate during encoder initialization.

## Explanation

Initialization fails and the triggering audio frame is not encoded. LAME receives the configured bitrate divided by 1000; FDK receives bits per second directly.

## Possible causes

- The selected library rejected the bitrate for the current audio format.

## What to check

1. Check the codec, sample rate, channel count, and requested bitrate.

## Recommended action

Use a bitrate accepted by the selected audio encoder.
