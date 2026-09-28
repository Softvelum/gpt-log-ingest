# Panel TCP socket could not be created

Product: Nimble Streamer
Component: Panel synchronization and proxy transport
Logging function: log_e
Log level: error
Documentation ID: log_e-409b20e7ba2dcf83

## Log pattern

```text
failed to create socket errno=%d
```

## Parameters

- Argument 1 — Operating system error: Operating system error code, integer; Windows uses its socket error codes for socket operations.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to create socket errno=5
```

## Short description

Panel TCP socket could not be created.

## Trigger condition

Creating the TCP socket fails while silent mode is disabled.

## Explanation

The panel connection helper returns an invalid socket.

## Possible causes

- The operating system rejected TCP socket creation.

## What to check

1. Check the socket error.

## Recommended action

Resolve the socket creation failure.
