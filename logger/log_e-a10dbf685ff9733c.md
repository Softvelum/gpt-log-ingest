# Could not create the HTTP request worker

Product: Nimble Streamer
Component: Server startup and services
Logging function: log_e
Log level: error
Documentation ID: log_e-a10dbf685ff9733c

## Log pattern

```text
failed to create worker thread %d
```

## Parameters

- Argument 1 — Worker index: Zero-based index of the worker whose creation failed.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to create worker thread 0
```

## Short description

Could not create the HTTP request worker.

## Trigger condition

A configured HTTP worker is being created. Thread creation reports failure.

## Explanation

The remaining server startup sequence is skipped and the server run routine returns failure. This path does not reach the normal worker-join sequence. The preceding thread resource error can provide the underlying reason.

## Possible causes

- Creation of the HTTP request worker failed.

## What to check

1. Inspect the preceding thread resource error for the failed HTTP request worker.

## Related log messages

- [Server worker thread could not start](log_e-7e7eeb5817c6f070.md)
