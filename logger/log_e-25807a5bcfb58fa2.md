# Icecast socket shutdown failed

Product: Nimble Streamer
Component: Icecast ingest and metadata
Logging function: log_e
Log level: error
Documentation ID: log_e-25807a5bcfb58fa2

## Log pattern

```text
failed to shutdown client s=%d (errno=%d)
```

## Parameters

- Argument 1 — Socket: Numeric connection socket descriptor.
- Argument 2 — Error: Operating-system error number at the failed shutdown.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to shutdown client s=42 (errno=107)
```

## Short description

Icecast socket shutdown failed.

## Trigger condition

A request to shut down an active Icecast connection fails.

## Explanation

The session is immediately marked shut down; cleanup can then close its socket.

## Possible causes

- A request to shut down an active Icecast connection fails.

## What to check

1. Inspect the operating-system error number and preceding connection failure.
