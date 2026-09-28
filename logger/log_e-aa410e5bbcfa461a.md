# Pending socket error could not be read

Product: Nimble Streamer
Component: Client and origin sockets
Logging function: log_e
Log level: error
Documentation ID: log_e-aa410e5bbcfa461a

## Log pattern

```text
getsockopt failed for %s s=%d errno=%d
```

## Parameters

- Argument 1 — Peer role: Text name identifying the socket role, such as client or origin.
- Argument 2 — Socket: Integer socket descriptor.
- Argument 3 — System error: Integer operating-system socket error value.

## Log examples

Illustrative examples; these are not captured server logs.

```text
getsockopt failed for origin s=12 errno=9
```

## Short description

Pending socket error could not be read.

## Trigger condition

Reading the socket pending-error option fails.

## Explanation

The output error status is set to a failure sentinel and the helper returns false.

## Possible causes

- The operating system rejected the socket option query.

## What to check

1. Inspect the socket and query error.
