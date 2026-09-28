# Could not create the advertising repackaging worker

Product: Nimble Streamer
Component: Server startup and services
Logging function: log_e
Log level: error
Documentation ID: log_e-e001e0cbd60f475c

## Log pattern

```text
failed to create advertising_transmuxer_thread thread %d
```

## Parameters

- Argument 1 — Worker index: Zero-based index of the worker whose creation failed.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to create advertising_transmuxer_thread thread 0
```

## Short description

Could not create the advertising repackaging worker.

## Trigger condition

An advertising URL is configured and one of its configured repackaging workers is being created. Thread creation reports failure.

## Explanation

The remaining server startup sequence is skipped and the server run routine returns failure. This path does not reach the normal worker-join sequence. The preceding thread resource error can provide the underlying reason.

## Possible causes

- Creation of the advertising repackaging worker failed.

## What to check

1. Inspect the preceding thread resource error for the failed advertising repackaging worker.

## Related log messages

- [Server worker thread could not start](log_e-7e7eeb5817c6f070.md)
