# AAC high bit rate index delta length is unsupported

Product: Nimble Streamer
Component: RTSP input and RTP media
Logging function: log_e
Log level: error
Documentation ID: log_e-e3b7dd7cf0a296de

## Log pattern

```text
AAC-hbr: unsupported indexdeltalength=%s

```

## Parameters

- Argument 1 — Advertised length: Original SDP field text; the expected numeric value is a bit count.

## Log examples

Illustrative examples; these are not captured server logs.

```text
AAC-hbr: unsupported indexdeltalength=2

```

## Short description

AAC high bit rate index delta length is unsupported.

## Trigger condition

The SDP AAC index delta length does not parse to 3.

## Explanation

The AAC high bit rate configuration is rejected before audio codec initialization.

## Possible causes

- The SDP AAC index delta length does not parse to 3.

## What to check

1. Check the AAC index delta length field in SDP.

## Recommended action

Use 3 bits for the AAC index delta length.
