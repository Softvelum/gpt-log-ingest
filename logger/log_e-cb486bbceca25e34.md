# Could not create the WMSPanel synchronization worker

Product: Nimble Streamer
Component: Server startup and services
Logging function: log_e
Log level: error
Documentation ID: log_e-cb486bbceca25e34

## Log pattern

```text
failed to create WMSPanel sync thread
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to create WMSPanel sync thread
```

## Short description

Could not create the WMSPanel synchronization worker.

## Trigger condition

The panel settings and statistics synchronization worker is being created. Thread creation reports failure.

## Explanation

The remaining server startup sequence is skipped and the server run routine returns failure. This path does not reach the normal worker-join sequence. The preceding thread resource error can provide the underlying reason.

## Possible causes

- Creation of the WMSPanel synchronization worker failed.

## What to check

1. Inspect the preceding thread resource error for the failed WMSPanel synchronization worker.

## Related log messages

- [Server worker thread could not start](log_e-7e7eeb5817c6f070.md)
