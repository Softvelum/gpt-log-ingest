# Could not create the scheduled stream checker

Product: Nimble Streamer
Component: Server startup and services
Logging function: log_e
Log level: error
Documentation ID: log_e-d48650dbf29b00fc

## Log pattern

```text
failed to create StreamChecker thread
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to create StreamChecker thread
```

## Short description

Could not create the scheduled stream checker.

## Trigger condition

The scheduled stream checker is being created in a Linux build. Thread creation reports failure.

## Explanation

The remaining server startup sequence is skipped and the server run routine returns failure. This path does not reach the normal worker-join sequence. The preceding thread resource error can provide the underlying reason.

## Possible causes

- Creation of the scheduled stream checker failed.

## What to check

1. Inspect the preceding thread resource error for the failed scheduled stream checker.

## Related log messages

- [Server worker thread could not start](log_e-7e7eeb5817c6f070.md)
