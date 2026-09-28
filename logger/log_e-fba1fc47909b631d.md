# RTMP old-style client-block echo cannot be built

Product: Nimble Streamer
Component: RTMP connection transport
Logging function: log_e
Log level: error
Documentation ID: log_e-fba1fc47909b631d

## Log pattern

```text
failed to build old-style s1 for client: %d
```

## Parameters

- Argument 1 — Socket: Operating-system socket descriptor for this connection.

## Log examples

No reachable illustrative example is established: the current caller reserves exactly the space required before this checked write.

## Short description

RTMP old-style client-block echo cannot be built.

## Trigger condition

The handshake builder reports insufficient output space while preparing the old-style client-block echo.

## Explanation

The incoming connection would be shut down. The checked path has already allocated and bounded the exact required space, so ordinary execution does not establish a reachable failure at this check.

## Possible causes

- The handshake builder reports insufficient output space while preparing the old-style client-block echo.

## What to check

1. Check the surrounding handshake logs if this internal size check is observed.

## Recommended action

Report the unexpected handshake construction failure to support.
