# IPv4 listener could not be bound

Product: Nimble Streamer
Component: Server startup and services
Logging function: log_e
Log level: error
Documentation ID: log_e-cbb2a072dcb33181

## Log pattern

```text
failed to bind to %s:%d s=%d errno=%d '%s'
```

## Parameters

- Argument 1 — Address: Configured interface address or wildcard.
- Argument 2 — Port: Configured integer port.
- Argument 3 — Socket: Integer listener descriptor.
- Argument 4 — System error: System error value at the failed bind.
- Argument 5 — Error detail: System error text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to bind to 192.0.2.10:443 s=12 errno=98 'Address already in use'
```

## Short description

IPv4 listener could not be bound.

## Trigger condition

Binding a newly created IPv4 TCP or HTTP/3 UDP listener to its configured address and port fails.

## Explanation

The process exits with a failure status; listener setup does not continue.

## Possible causes

- The operating system rejected binding the address and port.

## What to check

1. Inspect the address, port and specific system error.

## Recommended action

Resolve the reported binding error before starting the server.
