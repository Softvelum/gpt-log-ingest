# HLS GET request prepared

Product: Nimble Streamer
Component: HLS and MPEG-TS HTTP input
Logging function: log_d
Log level: debug
Documentation ID: log_d-0a23cceb02b96ce6

## Log pattern

```text
getting url='%s'
```

## Parameters

- Argument 1 — url: Current origin URL.

## Log examples

Illustrative examples; these are not captured server logs.

```text
getting url='https://origin.example/live.m3u8'
```

## Short description

HLS GET request prepared.

## Trigger condition

The HLS input has constructed request headers and is about to copy them into the output buffer.

## Explanation

This trace occurs before buffer storage and socket transmission. The current URL may identify a playlist, segment, key, or initialization segment.

## Related log messages

- [Cannot store HLS HTTP request](log_e-27a3d8fd33aaec53.md)
- [HLS origin request send failed](log_e-d3cbc42156dc8551.md)
