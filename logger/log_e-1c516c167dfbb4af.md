# Could not create the WHEP playback worker

Product: Nimble Streamer
Component: Server startup and services
Logging function: log_e
Log level: error
Documentation ID: log_e-1c516c167dfbb4af

## Log pattern

```text
failed to create whep_worker_thread thread
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to create whep_worker_thread thread
```

## Short description

Could not create the WHEP playback worker.

## Trigger condition

WHEP playback is enabled and one of its configured playback workers is being created. Thread creation reports failure.

## Explanation

The remaining server startup sequence is skipped and the server run routine returns failure. This path does not reach the normal worker-join sequence. The preceding thread resource error can provide the underlying reason.

## Possible causes

- Creation of the WHEP playback worker failed.

## What to check

1. Inspect the preceding thread resource error for the failed WHEP playback worker.

## Related log messages

- [Server worker thread could not start](log_e-7e7eeb5817c6f070.md)
