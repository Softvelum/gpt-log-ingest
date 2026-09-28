# Could not create the HTTP/3 packet listener

Product: Nimble Streamer
Component: Server startup and services
Logging function: log_e
Log level: error
Documentation ID: log_e-9a8df0d77c51ce43

## Log pattern

```text
failed to create quic listener thread
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to create quic listener thread
```

## Short description

Could not create the HTTP/3 packet listener.

## Trigger condition

HTTP/3 support is included and needed, and the configured worker-count times viewer-limit is nonzero. Thread creation reports failure.

## Explanation

The remaining server startup sequence is skipped and the server run routine returns failure. This path does not reach the normal worker-join sequence. The preceding thread resource error can provide the underlying reason.

## Possible causes

- Creation of the HTTP/3 packet listener failed.

## What to check

1. Inspect the preceding thread resource error for the failed HTTP/3 packet listener.

## Related log messages

- [Server worker thread could not start](log_e-7e7eeb5817c6f070.md)
