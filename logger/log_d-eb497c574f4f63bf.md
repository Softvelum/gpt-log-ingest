# RIST output local connection is accepted

Product: Nimble Streamer
Component: MPEG-TS stream registration and parsing
Logging function: log_d
Log level: debug
Documentation ID: log_d-eb497c574f4f63bf

## Log pattern

```text
accepted rist sender sock=%d, id=%s
```

## Parameters

- Argument 1 — Socket: Newly accepted local helper socket descriptor.
- Argument 2 — Identifier: Configured RIST sender identifier.

## Log examples

Illustrative examples; these are not captured server logs.

```text
accepted rist sender sock=42, id=output1
```

## Short description

RIST output local connection is accepted.

## Trigger condition

The RIST helper connects to the local listener and its accepted socket attaches successfully.

## Explanation

The output switches from listener to accepted socket. The output then starts sending available transport data. This describes the local helper connection, not proof of a remote RIST peer connection.

## Possible causes

- The RIST helper connects to the local listener and its accepted socket attaches successfully.

## What to check

1. Correlate the local socket and input/output identifier with helper startup and connection errors.

## Recommended action

No action is needed for an expected helper attachment.
