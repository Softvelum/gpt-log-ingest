# HTTP connection closure event

Product: Nimble Streamer
Component: HTTP client sessions
Logging function: log_v
Log level: verbose
Documentation ID: log_v-22b4ea67fd09d08d

## Log pattern

```text
connection closed by s=%d (ev=0x%x)
```

## Parameters

- Argument 1 — Socket descriptor: Integer identifying the affected local socket.
- Argument 2 — Event mask: Hexadecimal integer mask of reported socket events.

## Log examples

Illustrative examples; these are not captured server logs.

```text
connection closed by s=17 (ev=0x10)
```

## Short description

HTTP connection closure event.

## Trigger condition

A socket closure or hang-up event reaches an HTTP/1 session, or a hang-up or error reaches an HTTP/2 connection.

## Explanation

HTTP/1 applies closure handling for the current client or origin phase; HTTP/2 marks the connection shut down. This message alone does not establish whether a media response was complete.

## Possible causes

- A socket closure or hang-up event reaches an HTTP/1 session, or a hang-up or error reaches an HTTP/2 connection.

## What to check

1. Check adjacent response-completion or socket-error messages if the disconnection interrupted playback.

## Recommended action

Check adjacent response-completion or socket-error messages if the disconnection interrupted playback.
