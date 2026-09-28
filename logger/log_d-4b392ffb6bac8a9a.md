# IPv6 listener socket opened

Product: Nimble Streamer
Component: Socket communication
Logging function: log_d
Log level: debug
Documentation ID: log_d-4b392ffb6bac8a9a

## Log pattern

```text
opened ipv6 listener socket s=%d
```

## Parameters

- Argument 1 — Socket: Socket descriptor, integer; no unit.

## Log examples

Illustrative examples; these are not captured server logs.

```text
opened ipv6 listener socket s=12
```

## Short description

IPv6 listener socket opened.

## Trigger condition

The IPv6 listener socket is created before binding and listening.

## Explanation

This confirms allocation only; later setup operations can still fail.
