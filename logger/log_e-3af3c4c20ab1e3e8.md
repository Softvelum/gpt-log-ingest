# Could not create the managed-task worker

Product: Nimble Streamer
Component: Server startup and services
Logging function: log_e
Log level: error
Documentation ID: log_e-3af3c4c20ab1e3e8

## Log pattern

```text
failed to create task manager thread
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to create task manager thread
```

## Short description

Could not create the managed-task worker.

## Trigger condition

Managed tasks are enabled and their worker is being created in a Linux build. Thread creation reports failure.

## Explanation

The remaining server startup sequence is skipped and the server run routine returns failure. This path does not reach the normal worker-join sequence. The preceding thread resource error can provide the underlying reason.

## Possible causes

- Creation of the managed-task worker failed.

## What to check

1. Inspect the preceding thread resource error for the failed managed-task worker.

## Related log messages

- [Server worker thread could not start](log_e-7e7eeb5817c6f070.md)
