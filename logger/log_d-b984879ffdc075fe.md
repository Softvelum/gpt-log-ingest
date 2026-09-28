# WHEP helper listener ready

Product: Nimble Streamer
Component: WHEP playback helper
Logging function: log_d
Log level: debug
Documentation ID: log_d-b984879ffdc075fe

## Log pattern

```text
whep interface listening %s:%d
```

## Parameters

- Argument 1 — Listener address: Local IPv4 address used for the helper TCP listener, text.
- Argument 2 — Listener port: Bound TCP port number, integer; may be selected by the operating system.

## Log examples

Illustrative examples; these are not captured server logs.

```text
whep interface listening 127.0.0.1:45000
```

## Short description

WHEP helper listener ready.

## Trigger condition

The WHEP helper TCP socket is bound, nonblocking, and listening.

## Explanation

Nimble records the actual bound address and port and then starts the helper if automatically managed.
