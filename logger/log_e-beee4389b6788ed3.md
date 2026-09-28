# HTTP/3 socket family could not be queried

Product: Nimble Streamer
Component: Server startup and services
Logging function: log_e
Log level: error
Documentation ID: log_e-beee4389b6788ed3

## Log pattern

```text
failed to get socket domain, errno=%d socket=%d
```

## Parameters

- Argument 1 — System error: Platform socket-error result.
- Argument 2 — Socket: Integer HTTP/3 UDP socket descriptor.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to get socket domain, errno=22 socket=12
```

## Short description

HTTP/3 socket family could not be queried.

## Trigger condition

Querying the address family of an HTTP/3 UDP socket fails. This path requires HTTP/3 support and a configuration that needs it.

## Explanation

Socket-option setup returns failure and server initialization exits with a failure status.

## Possible causes

- Querying the address family of an HTTP/3 UDP socket fails.

## What to check

1. Inspect the failed option and socket error.
