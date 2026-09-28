# AAC high bit rate configuration text is too short

Product: Nimble Streamer
Component: RTSP input and RTP media
Logging function: log_e
Log level: error
Documentation ID: log_e-8a943cfe5aab79ad

## Log pattern

```text
AAC-hbr: unable to parse config=%s

```

## Parameters

- Argument 1 — Configuration: Original SDP AAC configuration string.

## Log examples

Illustrative examples; these are not captured server logs.

```text
AAC-hbr: unable to parse config=12

```

## Short description

AAC high bit rate configuration text is too short.

## Trigger condition

The SDP AAC configuration contains fewer than four hexadecimal characters.

## Explanation

Codec setup returns failure before converting pairs of hexadecimal characters into audio configuration bytes.

## Possible causes

- The SDP AAC configuration contains fewer than four hexadecimal characters.

## What to check

1. Check the AAC configuration text in SDP.

## Recommended action

Supply at least two bytes of AAC configuration as hexadecimal text.
