# NDI sender skipped overwritten buffered frames

Product: Nimble Streamer
Component: NDI media integration
Logging function: log_i
Log level: info
Documentation ID: log_i-5647dfec0ac06e55

## Log pattern

```text
buffer reader too slow
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
buffer reader too slow
```

## Short description

NDI sender skipped overwritten buffered frames.

## Trigger condition

The reader index falls behind the oldest entry still retained in the 100-slot frame buffer.

## Explanation

The read index advances to the oldest surviving item, skipping overwritten frames, and reading continues.

## Possible causes

- The reader index falls behind the oldest entry still retained in the 100-slot frame buffer.

## What to check

1. Check whether the output sender is keeping pace with incoming frames.

## Related log messages

- [NDI sender pacing fell more than a frame behind](log_i-7a3a90a357166c2b.md)
- [NDI sender buffer ran out of frames](log_i-e3ca2ae074f020e2.md)
