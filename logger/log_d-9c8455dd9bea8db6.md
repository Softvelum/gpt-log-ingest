# SRT event interest registered

Product: Nimble Streamer
Component: SRT transport
Logging function: log_d
Log level: debug
Documentation ID: log_d-9c8455dd9bea8db6

## Log pattern

```text
srt_epoll_add_usock epfd=%d OK for fd=%d flags=0x%x
```

## Parameters

- Argument 1 — Poller: SRT event-poller identifier.
- Argument 2 — Socket: SRT socket or group identifier.
- Argument 3 — Events: SRT event-interest bit mask in hexadecimal; input is 0x1, output 0x4, and error 0x8.

## Log examples

Illustrative examples; these are not captured server logs.

```text
srt_epoll_add_usock epfd=1 OK for fd=100 flags=0x9
```

## Short description

SRT event interest registered.

## Trigger condition

The SRT library accepts the request to register a socket and its event-interest flags.

## Explanation

The operation reports success and the selected events can be monitored.

## Possible causes

- The SRT library accepts the request to register a socket and its event-interest flags.

## What to check

1. Match the socket and event mask with the expected connection state.
