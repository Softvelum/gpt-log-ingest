# AAC channel map loses channels

Product: Nimble Streamer
Component: FDK AAC audio decoding
Logging function: log_e
Log level: error
Documentation ID: log_e-bb51b5c0d3aa54d2

## Log pattern

```text
unsupported channel configuration, channel_count=%d -> channel_mask=%x
```

## Parameters

- Argument 1 — Channels: Number of audio channels, integer.
- Argument 2 — Channel mask: Constructed channel-location bit mask; supplied as a 64-bit value but printed with a hexadecimal integer conversion.

The mask argument is 64-bit while the format requests an ordinary hexadecimal integer; the displayed mask may omit high bits.

## Log examples

Illustrative examples; these are not captured server logs.

```text
unsupported channel configuration, channel_count=2 -> channel_mask=3
```

## Short description

AAC channel map loses channels.

## Trigger condition

The number of channels represented by the constructed channel mask differs from the decoded stream channel count.

## Explanation

The layout is rejected and packet processing fails.

## Possible causes

- The stream contains channel types that cannot be represented by the supported mapping.

## What to check

1. Compare the decoded channel count with the constructed mask.

## Recommended action

Use a supported AAC channel arrangement.
