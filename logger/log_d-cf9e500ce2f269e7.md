# RTSP RTCP connection reported open

Product: Nimble Streamer
Component: RTSP connections and control
Logging function: log_d
Log level: debug
Documentation ID: log_d-cf9e500ce2f269e7

## Log pattern

```text
rtsp s=%d added rtcp s=%d
```

## Parameters

- Argument 1 — RTSP socket: Parent control-session socket descriptor.
- Argument 2 — RTCP socket: Newly opened RTCP socket descriptor.

## Log examples

Illustrative examples; these are not captured server logs.

```text
rtsp s=25 added rtcp s=26
```

## Short description

RTSP RTCP connection reported open.

## Trigger condition

The new RTCP connection reports its socket to the parent RTSP input.

## Explanation

The parent looks up the media-track association and stores the connection when found. A missing association instead requests shutdown, so this message alone does not prove registration succeeded.

## Possible causes

- Normal callback during UDP RTCP connection creation.

## What to check

1. If setup stops afterward, check whether the socket has a media-track association.

## Related log messages

- [RTSP UDP connection pair queued](log_d-8c485dd9833be48f.md)
