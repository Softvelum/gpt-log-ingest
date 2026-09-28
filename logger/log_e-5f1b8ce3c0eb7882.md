# MPEG-TS input event threw an exception

Product: Nimble Streamer
Component: Live input and publishing workers
Logging function: log_e
Log level: error
Documentation ID: log_e-5f1b8ce3c0eb7882

## Log pattern

```text
unknown exception while processing session=%p (s=%d)
```

## Parameters

- Argument 1 — camera: Camera object memory address.
- Argument 2 — socket: Event socket descriptor.

## Log examples

Illustrative examples; these are not captured server logs.

```text
unknown exception while processing session=0x1000 (s=12)
```

## Short description

MPEG-TS input event threw an exception.

## Trigger condition

Any exception escapes camera event processing.

## Explanation

The camera is removed from its task and socket maps, and the configuration marker is cleared so missing inputs can be recreated. The message supplies no exception description.

## Possible causes

- An exception interrupted camera event processing.

## What to check

1. Correlate the camera address and socket with earlier input diagnostics.

## Recommended action

Investigate the operation that raised the exception.
