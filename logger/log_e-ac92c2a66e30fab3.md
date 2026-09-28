# Could not create the MPEG-TS over RIST output worker

Product: Nimble Streamer
Component: Server startup and services
Logging function: log_e
Log level: error
Documentation ID: log_e-ac92c2a66e30fab3

## Log pattern

```text
failed to create mpeg2ts_rist_sender thread
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to create mpeg2ts_rist_sender thread
```

## Short description

Could not create the MPEG-TS over RIST output worker.

## Trigger condition

RIST support is compiled in and its output worker is being created. Thread creation reports failure.

## Explanation

The remaining server startup sequence is skipped and the server run routine returns failure. This path does not reach the normal worker-join sequence. The preceding thread resource error can provide the underlying reason.

## Possible causes

- Creation of the MPEG-TS over RIST output worker failed.

## What to check

1. Inspect the preceding thread resource error for the failed MPEG-TS over RIST output worker.

## Related log messages

- [Server worker thread could not start](log_e-7e7eeb5817c6f070.md)
