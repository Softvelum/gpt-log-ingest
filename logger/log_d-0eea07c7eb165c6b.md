# HTTP socket event received

Product: Nimble Streamer
Component: HTTP client sessions
Logging function: log_d
Log level: debug
Documentation ID: log_d-0eea07c7eb165c6b

## Log pattern

```text
[CliS::PE] p=%p st=%d s=%d ev=0x%x ssl_o=%d
```

## Parameters

- Argument 1 — Session address: Pointer identifying this client session within the process.
- Argument 2 — Session state: Integer identifying the HTTP client session phase: 0 reading a request, 1 sending a response, 2 connecting to origin, 3 sending to origin, 4 reading origin headers, 5 reading an origin playlist, 6 reading origin media, 29 shutting down, or 30 finished; intermediate values identify pending asynchronous work.
- Argument 3 — Socket descriptor: Integer identifying the affected local socket.
- Argument 4 — Event mask: Hexadecimal integer mask of reported socket events.
- Argument 5 — Origin TLS flag: Boolean rendered as 1 for an HTTPS origin and 0 otherwise.

## Log examples

Illustrative examples; these are not captured server logs.

```text
[CliS::PE] p=0x1234 st=4 s=17 ev=0x10 ssl_o=1
```

## Short description

HTTP socket event received.

## Trigger condition

The HTTP client session receives a socket event.

## Explanation

The activity time is updated and the event is dispatched according to connection state and TLS direction.

## Possible causes

- The HTTP client session receives a socket event.

## What to check

1. Not applicable; this is normal diagnostic progress. Use the socket identifier to follow the same request if troubleshooting.

## Recommended action

Not applicable; this is normal diagnostic progress. Use the socket identifier to follow the same request if troubleshooting.
