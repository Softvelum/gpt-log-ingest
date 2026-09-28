# SRT publisher stream ID cannot be parsed

Product: Nimble Streamer
Component: SRT publishing authentication
Logging function: log_e
Log level: error
Documentation ID: log_e-6c1c97be0b7ad4aa

## Log pattern

```text
[srt_listen_callback] invalid streamid=%s,ip=[%s]
```

## Parameters

- Argument 1 — Stream ID: Requested SRT stream identifier.
- Argument 2 — Peer: Publisher IP address.

## Log examples

Illustrative examples; these are not captured server logs.

```text
[srt_listen_callback] invalid streamid=news,ip=[192.0.2.20]
```

## Short description

SRT publisher stream ID cannot be parsed.

## Trigger condition

The handshake stream ID has fewer than two slash-separated target components.

## Explanation

The authenticated listener rejects the incoming connection.

## Possible causes

- The handshake stream ID has fewer than two slash-separated target components.

## What to check

1. Check the application/stream structure of the publisher stream ID.

## Recommended action

Supply a valid application/stream identifier.
