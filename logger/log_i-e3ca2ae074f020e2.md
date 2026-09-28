# NDI sender buffer ran out of frames

Product: Nimble Streamer
Component: NDI media integration
Logging function: log_i
Log level: info
Documentation ID: log_i-e3ca2ae074f020e2

## Log pattern

```text
buffer empty, wait for buffer refill
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
buffer empty, wait for buffer refill
```

## Short description

NDI sender buffer ran out of frames.

## Trigger condition

The buffer was initialized but its read and write positions become equal.

## Explanation

Initial buffering and pacing state are reset, then the sender waits for more frames. The message indicates a temporary underrun rather than immediate shutdown.

## Possible causes

- The buffer was initialized but its read and write positions become equal.

## What to check

1. Check whether upstream audio or video delivery has stopped or become intermittent.

## Related log messages

- [NDI sender slows down to rebuild buffering](log_i-316b14400db2cd6f.md)
- [NDI sender skipped overwritten buffered frames](log_i-5647dfec0ac06e55.md)
