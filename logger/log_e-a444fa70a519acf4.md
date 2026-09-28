# Could not create the advertising download worker

Product: Nimble Streamer
Component: Server startup and services
Logging function: log_e
Log level: error
Documentation ID: log_e-a444fa70a519acf4

## Log pattern

```text
failed to create advertising_download_thread thread
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to create advertising_download_thread thread
```

## Short description

Could not create the advertising download worker.

## Trigger condition

An advertising URL is configured and its download worker is being created. Thread creation reports failure.

## Explanation

The remaining server startup sequence is skipped and the server run routine returns failure. This path does not reach the normal worker-join sequence. The preceding thread resource error can provide the underlying reason.

## Possible causes

- Creation of the advertising download worker failed.

## What to check

1. Inspect the preceding thread resource error for the failed advertising download worker.

## Related log messages

- [Server worker thread could not start](log_e-7e7eeb5817c6f070.md)
