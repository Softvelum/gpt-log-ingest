# AAC RTP packet is shorter than its declared header area

Product: Nimble Streamer
Component: RTSP input and RTP media
Logging function: log_e
Log level: error
Documentation ID: log_e-cc121e50d6bfcd18

## Log pattern

```text
len=%d must be grater then au_headers_length=%d

```

## Parameters

- Argument 1 — First printed number: Receives the declared header-area byte count despite the len label.
- Argument 2 — Second printed number: No argument is supplied; its value is not determined from source.

The format has two integer conversions but the call supplies only one argument. Formatted output is therefore unreliable.

## Log examples

No sound example can be produced because the logging call supplies only one value for two integer conversions.

## Short description

AAC RTP packet is shorter than its declared header area.

## Trigger condition

After reading the two-byte access-unit-header-length field, the remaining packet size is smaller than the declared header area in bytes.

## Explanation

The RTP packet is rejected as a parse failure. The logging call itself is malformed: its first conversion receives the header-area byte count, while no value is supplied for the second conversion. Neither printed label can be treated as a reliable pair of sizes.

## Possible causes

- After reading the two-byte access-unit-header-length field, the remaining packet size is smaller than the declared header area in bytes.

## What to check

1. Check the actual RTP payload length against its declared AAC header length.

## Recommended action

Provide complete AAC RTP packets; do not diagnose the size difference from the two printed numbers.

## Related log messages

- [AAC RTP header length is not a multiple of sixteen bits](log_e-690b1507d0635b85.md)
