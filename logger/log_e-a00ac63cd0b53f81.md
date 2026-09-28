# AAC channel group is unsupported

Product: Nimble Streamer
Component: FDK AAC audio decoding
Logging function: log_e
Log level: error
Documentation ID: log_e-a00ac63cd0b53f81

## Log pattern

```text
unsupported channel layout, channel_type=%d, channel_count=%d
```

## Parameters

- Argument 1 — Channel group: FDK channel-type identifier, integer.
- Argument 2 — Group count: Channels in this type of channel group, integer.

## Log examples

Illustrative examples; these are not captured server logs.

```text
unsupported channel layout, channel_type=1, channel_count=4
```

## Short description

AAC channel group is unsupported.

## Trigger condition

The decoded front, side, back, or low-frequency channel count has no supported mapping.

## Explanation

Channel-layout construction fails and packet processing reports failure. Supported groups are 1-3 front, 2 side, 1-3 back, and 1 low-frequency channel.

## Possible causes

- A channel group has an unsupported channel count.

## What to check

1. Check the reported channel-group type and count.

## Recommended action

Use a supported AAC channel arrangement.
