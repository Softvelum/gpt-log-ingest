# WHIP frame sizes invalid

Product: Nimble Streamer
Component: WHIP publishing helper
Logging function: log_e
Log level: error
Documentation ID: log_e-ec468cbde1308d15

## Log pattern

```text
failed to read frame for session=%d, stream=[%s/%s]. Invalid header. frame info length=%d. frame length=%d
```

## Parameters

- Argument 1 — Session identifier: WebRTC helper session identifier, unsigned 32-bit integer displayed in decimal; no unit.
- Argument 2 — Application: Application name, text.
- Argument 3 — Stream: Stream name, text.
- Argument 4 — Information length: Frame-information body length, bytes, unsigned 32-bit integer displayed in decimal.
- Argument 5 — Media length: Encoded frame payload length after subtracting information bytes, bytes, unsigned 32-bit integer displayed in decimal.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to read frame for session=7, stream=[live/camera]. Invalid header. frame info length=80. frame length=1048577
```

## Short description

WHIP frame sizes invalid.

## Trigger condition

After subtracting frame-information length from message length, the information or media length is zero, or the media payload exceeds the 1,048,576-byte frame buffer limit.

## Explanation

The frame is rejected and the caller removes the publication and closes its socket. The preceding total-length check normally catches a zero media length first.

## Possible causes

- The helper supplies an empty information block or a media payload larger than the accepted frame limit.

## What to check

1. Inspect both lengths and the upstream encoded frame size.

## Recommended action

Correct malformed framing or reduce oversized encoded frames, then reconnect.
