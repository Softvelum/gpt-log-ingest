# HTTP/2 deferred ping limit reached

Product: Nimble Streamer
Component: HTTP/2 connection
Logging function: log_v
Log level: verbose
Documentation ID: log_v-7d612bdd8e2f937d

## Log pattern

```text
too many ping requests emitted by s=%d
```

## Parameters

- Argument 1 — Socket: Socket descriptor, integer; no unit.

## Log examples

Illustrative examples; these are not captured server logs.

```text
too many ping requests emitted by s=12
```

## Short description

HTTP/2 deferred ping limit reached.

## Trigger condition

An HTTP/2 ping cannot be replied to immediately and cannot enter the deferred ping queue: another deferred ping has the same activity timestamp, or three deferred pings are already queued.

## Explanation

Nimble attempts to send or queue a GOAWAY with the excessive-load error code, then reports input processing failure so the connection is shut down.

## Possible causes

- The client sends pings too quickly while output is busy, or deferred ping replies accumulate.

## What to check

1. Check the client ping cadence and whether outbound responses are blocked.

## Recommended action

Reduce client ping frequency and resolve prolonged output stalls.
