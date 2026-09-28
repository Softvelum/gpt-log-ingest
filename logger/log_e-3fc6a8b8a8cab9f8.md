# Origin connection completion failed

Product: Nimble Streamer
Component: HTTP client sessions
Logging function: log_e
Log level: error
Documentation ID: log_e-3fc6a8b8a8cab9f8

## Log pattern

```text
failed to connect to origin='%s' (s=%d state=%d)
```

## Parameters

- Argument 1 — Origin host: Text containing the origin hostname and optional port.
- Argument 2 — Origin socket: Integer identifying the local origin socket.
- Argument 3 — Session state: Integer identifying the HTTP client session phase: 0 reading a request, 1 sending a response, 2 connecting to origin, 3 sending to origin, 4 reading origin headers, 5 reading an origin playlist, 6 reading origin media, 29 shutting down, or 30 finished; intermediate values identify pending asynchronous work.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to connect to origin='media.example:8080' (s=17 state=4)
```

## Short description

Origin connection completion failed.

## Trigger condition

An origin connection has a nonzero socket error or closes during connection/request transmission.

## Explanation

The session attempts the next configured origin if one remains; otherwise it prepares HTTP 500.

## Possible causes

- An origin connection has a nonzero socket error or closes during connection/request transmission.

## What to check

1. Check origin reachability and adjacent socket errors; confirm a usable alternate origin exists if continuity is required.

## Recommended action

Check origin reachability and adjacent socket errors; confirm a usable alternate origin exists if continuity is required.
