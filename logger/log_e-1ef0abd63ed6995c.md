# HTTP/3 transport library initialization failed

Product: Nimble Streamer
Component: Server startup and services
Logging function: log_e
Log level: error
Documentation ID: log_e-1ef0abd63ed6995c

## Log pattern

```text
cannot init lsquic
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
cannot init lsquic
```

## Short description

HTTP/3 transport library initialization failed.

## Trigger condition

HTTP/3 support is included and needed by the configured TLS service, but global transport initialization fails.

## Explanation

The process exits with a failure status before creating HTTP/3 TLS contexts or UDP listeners.

## Possible causes

- The HTTP/3 transport library rejected global initialization.

## What to check

1. Check preceding HTTP/3 initialization diagnostics.
