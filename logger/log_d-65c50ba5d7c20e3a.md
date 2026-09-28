# WHIP helper listener ready

Product: Nimble Streamer
Component: WHIP publishing helper
Logging function: log_d
Log level: debug
Documentation ID: log_d-65c50ba5d7c20e3a

## Log pattern

```text
whip interface listening %s:%d
```

## Parameters

- Argument 1 — Listener address: Local IPv4 address used for the helper TCP listener, text.
- Argument 2 — Listener port: Bound TCP port number, integer; may be selected by the operating system.

## Log examples

Illustrative examples; these are not captured server logs.

```text
whip interface listening 127.0.0.1:45000
```

## Short description

WHIP helper listener ready.

## Trigger condition

The WHIP helper TCP socket has been bound, made nonblocking, and placed in listening mode.

## Explanation

Nimble records the actual local address and port, then attempts to start the helper if it is managed automatically.
