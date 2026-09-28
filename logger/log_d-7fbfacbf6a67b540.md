# Icecast server thread started

Product: Nimble Streamer
Component: Icecast ingest and metadata
Logging function: log_d
Log level: debug
Documentation ID: log_d-7fbfacbf6a67b540

## Log pattern

```text
IcecastServerThread started
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
IcecastServerThread started
```

## Short description

Icecast server thread started.

## Trigger condition

The Icecast server thread enters its main routine.

## Explanation

Interface synchronization and socket processing will run until shutdown is requested. No bound endpoint is confirmed by this message.

## Possible causes

- The Icecast server thread enters its main routine.

## What to check

1. Check subsequent listener setup diagnostics.
