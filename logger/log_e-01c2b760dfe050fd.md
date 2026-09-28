# Audio message does not contain the required data

Product: Nimble Streamer
Component: Audio input
Logging function: log_e
Log level: error
Documentation ID: log_e-01c2b760dfe050fd

## Log pattern

```text
[AM] wrong message size %d
```

## Parameters

- Argument 1 — Message bytes: Number of bytes in the supplied media message.

## Log examples

Illustrative examples; these are not captured server logs.

```text
[AM] wrong message size 1
```

## Short description

Audio message does not contain the required data.

## Trigger condition

DVR AAC data lacks its packet-type byte, or RTMP audio lacks a required header/configuration field or contains invalid AC3/EAC3 framing.

## Explanation

DVR processing and most RTMP branches return failure; the RTMP AC3 branch discards an empty or partial frame set without failing the session.

## Possible causes

- DVR AAC data lacks its packet-type byte, or RTMP audio lacks a required header/configuration field or contains invalid AC3/EAC3 framing.

## What to check

1. Check complete audio headers and AC3/EAC3 frame boundaries.

## Recommended action

Check complete audio headers and AC3/EAC3 frame boundaries.
