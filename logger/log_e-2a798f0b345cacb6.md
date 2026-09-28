# SRT handshake has no stream-ID value

Product: Nimble Streamer
Component: SRT publishing authentication
Logging function: log_e
Log level: error
Documentation ID: log_e-2a798f0b345cacb6

## Log pattern

```text
[srt_listen_callback] invalid streamid=null,ip=[%s]
```

## Parameters

- Argument 1 — Peer: Publisher IP address.

## Log examples

Illustrative examples; these are not captured server logs.

```text
[srt_listen_callback] invalid streamid=null,ip=[192.0.2.20]
```

## Short description

SRT handshake has no stream-ID value.

## Trigger condition

The authenticated listener callback receives a null stream-ID argument.

## Explanation

The callback rejects the incoming connection before parsing its target stream.

## Possible causes

- The authenticated listener callback receives a null stream-ID argument.

## What to check

1. Check the publisher handshake and stream-ID support.
