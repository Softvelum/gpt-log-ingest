# Could not create the SRT multipoint output listener

Product: Nimble Streamer
Component: Server startup and services
Logging function: log_e
Log level: error
Documentation ID: log_e-050be4fb97f4bae9

## Log pattern

```text
failed to create mpeg2ts_srt_server thread
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to create mpeg2ts_srt_server thread
```

## Short description

Could not create the SRT multipoint output listener.

## Trigger condition

SRT support is included and available, multipoint listening is enabled, and one of its configured output listeners is being created. Thread creation reports failure.

## Explanation

The remaining server startup sequence is skipped and the server run routine returns failure. This path does not reach the normal worker-join sequence. The preceding thread resource error can provide the underlying reason.

## Possible causes

- Creation of the SRT multipoint output listener failed.

## What to check

1. Inspect the preceding thread resource error for the failed SRT multipoint output listener.

## Related log messages

- [Server worker thread could not start](log_e-7e7eeb5817c6f070.md)
