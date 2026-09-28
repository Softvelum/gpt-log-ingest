# Client request read reported closure

Product: Nimble Streamer
Component: HTTP client sessions
Logging function: log_d
Log level: debug
Documentation ID: log_d-70ecaa635a37db1a

## Log pattern

```text
received=0 (connection closed by client s=%d)
```

## Parameters

- Argument 1 — Client socket: Integer identifying the local client socket.

## Log examples

Illustrative examples; these are not captured server logs.

```text
received=0 (connection closed by client s=18)
```

## Short description

Client request read reported closure.

## Trigger condition

Reading a request reaches the zero-byte connection-closure branch.

## Explanation

The client-closure handler runs and applies the current session cleanup policy.

## Possible causes

- Reading a request reaches the zero-byte connection-closure branch.

## What to check

1. Check whether the client intentionally closed the connection before completing its request.

## Recommended action

Check whether the client intentionally closed the connection before completing its request.
