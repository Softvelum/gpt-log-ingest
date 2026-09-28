# Log-upload compression sizes

Product: Nimble Streamer
Component: WMSPanel synchronization
Logging function: log_d
Log level: debug
Documentation ID: log_d-64edf3ff4c0f17c5

## Log pattern

```text
log_body.size()=%llu read_size=%d
```

## Parameters

- Argument 1 — compressed\_bytes: Compressed log-body length in bytes, cast to an unsigned 64-bit integer.
- Argument 2 — read\_bytes: Uncompressed bytes counted for the retained lines including line-feed bytes.

The first conversion is %llu but its argument is uint64_t, whose underlying type can be unsigned long on some targets. The conversion is portable only when that type is unsigned long long.

## Log examples

No target-independent example is given because the 64-bit length conversion depends on the platform integer type.

## Short description

Log-upload compression sizes.

## Trigger condition

Log-upload preparation has finished reading and compressing the available log tail, or failed to open it.

## Explanation

The reader considers at most the final 10 MiB, drops the first line, and compresses following complete lines. Zero read bytes selects the explanatory text response instead of an attachment.
