# Peer socket address is unavailable

Product: Nimble Streamer
Component: Socket communication
Logging function: log_v
Log level: verbose
Documentation ID: log_v-e3deabf513c051b0

## Log pattern

```text
getpeername s=%d failed errno=%d
```

## Parameters

- Argument 1 — Socket: Socket descriptor, integer; no unit.
- Argument 2 — Operating system error: Operating system error code, integer; Windows uses its socket error codes for socket operations.

## Log examples

Illustrative examples; these are not captured server logs.

```text
getpeername s=12 failed errno=5
```

## Short description

Peer socket address is unavailable.

## Trigger condition

Reading the peer address fails because the socket is no longer connected.

## Explanation

The peer-address helper returns empty text.

## Possible causes

- A peer can disconnect before Nimble starts processing its request.

## What to check

1. Check the socket error and whether the peer remains connected.

## Recommended action

Investigate persistent peer-address failures; an already disconnected peer requires no address lookup.

## Related log messages

- [Peer socket address is unavailable](log_e-8838eff15a90514b.md)
