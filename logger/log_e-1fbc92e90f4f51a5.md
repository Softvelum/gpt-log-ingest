# Cannot bind Icecast listening endpoint

Product: Nimble Streamer
Component: Icecast ingest and metadata
Logging function: log_e
Log level: error
Documentation ID: log_e-1fbc92e90f4f51a5

## Log pattern

```text
failed to bind icecast socket %s:%d
```

## Parameters

- Argument 1 — Address: Configured local listening IP address.
- Argument 2 — Port: Configured local TCP port.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to bind icecast socket 0.0.0.0:8000
```

## Short description

Cannot bind Icecast listening endpoint.

## Trigger condition

Creating a listening socket for a new configured Icecast interface fails.

## Explanation

That endpoint is unavailable. The interface settings version is not accepted, allowing synchronization to retry later. Other valid interfaces can remain active.

## Possible causes

- Creating a listening socket for a new configured Icecast interface fails.

## What to check

1. Check the local address, port, and preceding socket diagnostics.
