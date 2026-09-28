# HTTP/2 connection preface invalid

Product: Nimble Streamer
Component: HTTP/2 connection
Logging function: log_v
Log level: verbose
Documentation ID: log_v-321f67beea3ff6f6

## Log pattern

```text
invalid h2 preface by s=%d
```

## Parameters

- Argument 1 — Socket: Socket descriptor, integer; no unit.

## Log examples

Illustrative examples; these are not captured server logs.

```text
invalid h2 preface by s=12
```

## Short description

HTTP/2 connection preface invalid.

## Trigger condition

One or more buffered preface bytes differ from the expected 24-byte HTTP/2 client preface, even if the preface is still incomplete.

## Explanation

Preface processing fails and the caller shuts down the connection.

## Possible causes

- The peer sends incorrect HTTP/2 startup bytes or uses another protocol on the connection.

## What to check

1. Check client protocol negotiation and the first bytes sent after connection setup.

## Recommended action

Correct the client protocol or endpoint configuration.
