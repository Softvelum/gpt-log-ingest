# SRT socket removed from event poller

Product: Nimble Streamer
Component: SRT transport
Logging function: log_d
Log level: debug
Documentation ID: log_d-066ded5fa38a7a5e

## Log pattern

```text
srt_epoll_remove_usock epfd=%d OK for fd=%d
```

## Parameters

- Argument 1 — Poller: SRT event-poller identifier.
- Argument 2 — Socket: SRT socket or group identifier.

## Log examples

Illustrative examples; these are not captured server logs.

```text
srt_epoll_remove_usock epfd=1 OK for fd=100
```

## Short description

SRT socket removed from event poller.

## Trigger condition

The SRT library accepts removing the socket from the poller.

## Explanation

The removal operation returns success.

## Possible causes

- The SRT library accepts removing the socket from the poller.

## What to check

1. Check the socket identity and prior registration or removal messages.
