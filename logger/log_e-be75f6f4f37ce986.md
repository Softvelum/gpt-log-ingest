# Scenario input sender fell behind

Product: Nimble Streamer
Component: Scenario input forwarding
Logging function: log_e
Log level: error
Documentation ID: log_e-be75f6f4f37ce986

## Log pattern

```text
failed to send remote transcode stream in appropriate rate from [%s/%s]
```

## Parameters

- Argument 1 — Application: Application name, text.
- Argument 2 — Stream: Stream name, text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to send remote transcode stream in appropriate rate from [live/camera]
```

## Short description

Scenario input sender fell behind.

## Trigger condition

The next requested input item is older than the earliest item retained by the live stream buffer.

## Explanation

The sender resets its read position to the decoder starting point and skips video until a keyframe; forwarding can continue with a gap in media.

## Possible causes

- Scenario input consumption is slower than the rate at which old items leave the live buffer.

## What to check

1. Check parent/child processing load, input rate and live-buffer retention.

## Recommended action

Restore timely scenario input consumption or reduce the load; increase retention only if the added delay is acceptable.
