# Stream-check socket creation failed

Product: Nimble Streamer
Component: Linux stream availability checks
Logging function: log_e
Log level: error
Documentation ID: log_e-c96c07eaba745a01

## Log pattern

```text
checker socket creation failed (errno=%d)
```

## Parameters

- Argument 1 — error: Operating system error number.

## Log examples

Illustrative examples; these are not captured server logs.

```text
checker socket creation failed (errno=24)
```

## Short description

Stream-check socket creation failed.

## Trigger condition

Creating an IPv4 TCP socket fails for the first or an additional resolved address.

## Explanation

The current connection attempt returns failure before nonblocking setup or connecting.

## Possible causes

- The operating system could not create the socket.

## What to check

1. Check the error number and available socket or file-descriptor resources.

## Recommended action

Resolve the socket resource failure.
