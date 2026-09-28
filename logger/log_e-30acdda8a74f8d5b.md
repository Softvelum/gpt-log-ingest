# SDP H.264 sequence parameter set decodes to too few bytes

Product: Nimble Streamer
Component: RTSP input and RTP media
Logging function: log_e
Log level: error
Documentation ID: log_e-30acdda8a74f8d5b

## Log pattern

```text
H264: unable to parse(< 4 bytes) sprop-parameter-sets=%s

```

## Parameters

- Argument 1 — Parameter-set text: Original SDP sprop-parameter-sets text, before Base64 decoding.

## Log examples

Illustrative examples; these are not captured server logs.

```text
H264: unable to parse(< 4 bytes) sprop-parameter-sets=AA==,AA==

```

## Short description

SDP H.264 sequence parameter set decodes to too few bytes.

## Trigger condition

The first comma-separated SDP parameter set decodes to fewer than four bytes.

## Explanation

H.264 setup returns failure. A missing comma or missing picture parameter set follows a separate path that waits for parameter sets in media packets.

## Possible causes

- The first comma-separated SDP parameter set decodes to fewer than four bytes.

## What to check

1. Check the first Base64 value in the SDP parameter-set pair.

## Recommended action

Provide complete H.264 parameter sets in SDP or in the incoming media.

## Related log messages

- [H.264 sequence parameter set is too short](log_e-5188d8b08873bff1.md)
