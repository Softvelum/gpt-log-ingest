# Incoming connection accepted

Product: Nimble Streamer
Component: RTMP, RTSP, and HTTP management listeners
Logging function: log_d
Log level: debug
Documentation ID: log_d-f24ac638c12cbb90

## Log pattern

```text
accepted s=%d (%s:%d)
```

## Parameters

- Argument 1 — socket: Newly accepted socket descriptor.
- Argument 2 — client\_ip: Peer IP address.
- Argument 3 — client\_port: Peer TCP port number.

## Log examples

Illustrative examples; these are not captured server logs.

```text
accepted s=42 (192.0.2.10:51000)
```

## Short description

Incoming connection accepted.

## Trigger condition

A listener has accepted a new socket and debug logging is enabled.

## Explanation

RTMP and management listeners print this before constructing their client and before optional TLS initialization. The RTSP listener prints it after creating and registering its session. It confirms socket acceptance, not successful authentication or media playback.
