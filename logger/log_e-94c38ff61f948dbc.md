# RTSP SDP could not obtain H.264 parameter sets

Product: Nimble Streamer
Component: RTSP connections and control
Logging function: log_e
Log level: error
Documentation ID: log_e-94c38ff61f948dbc

## Log pattern

```text
failed to get avc params, client: %d
```

## Parameters

- Argument 1 — Socket: Integer operating-system socket descriptor identifying this connection.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to get avc params, client: 25
```

## Short description

RTSP SDP could not obtain H.264 parameter sets.

## Trigger condition

Building an SDP description for H.264 fails while extracting parameter sets from the stored video sequence header.

## Explanation

SDP construction returns failure. Truncated header fields or declared parameter-set lengths beyond the stored bytes are rejected.

## Possible causes

- Building an SDP description for H.264 fails while extracting parameter sets from the stored video sequence header.

## What to check

1. Check the stored H.264 sequence header for complete parameter sets.

## Related log messages

- [RTSP H.265 parameter extraction failure branch](log_e-0d20795601ea832d.md)
- [RTSP SDP found no supported media codec](log_e-e9275ab441f28ae0.md)
