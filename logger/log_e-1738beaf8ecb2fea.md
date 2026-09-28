# Video message is too short

Product: Nimble Streamer
Component: Video packet validation
Logging function: log_e
Log level: error
Documentation ID: log_e-1738beaf8ecb2fea

## Log pattern

```text
[VM] wrong message size %d
```

## Parameters

- Argument 1 — Buffer size: Unsigned byte count printed in decimal. Most occurrences report total message size; low-level AVC resolution parsing reports configuration size, and after its five-byte prefix is skipped it reports the remaining configuration size.

Twenty occurrences: AVC resolution checks require six configuration bytes, then two-byte SPS lengths and their complete contents; the RTMP-wrapped helper requires eleven total bytes. DVR checks the frame/packet/composition headers, AVC profile fields and SPS/PPS lengths and data, or a complete variable RTMP header. RTMP ingest checks a nonempty message, enhanced headers with payload beyond five bytes (or eight for HEVC coded frames), and legacy packet/composition fields.

## Log examples

Illustrative examples; these are not captured server logs.

```text
[VM] wrong message size 4
```

## Short description

Video message is too short.

## Trigger condition

A video header or declared parameter-set length extends beyond the supplied buffer according to a size check.

## Explanation

The active parser or DVR/RTMP video operation returns failure before it can finish that message. Resolution extraction does not provide a usable result.

## Possible causes

- A video message or codec configuration is empty, truncated, or declares parameter-set lengths larger than its available data.

## What to check

1. Check the source message framing and AVC configuration lengths; identify whether the surrounding operation is resolution extraction, DVR recording, or RTMP ingest.

## Recommended action

Correct incomplete or malformed video messages at the publisher or input source.
