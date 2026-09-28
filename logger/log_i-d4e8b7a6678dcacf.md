# TLS certificate reload requested

Product: Nimble Streamer
Component: TLS certificate management
Logging function: log_i
Log level: info
Documentation ID: log_i-d4e8b7a6678dcacf

## Log pattern

```text
reloading SSL certificates
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
reloading SSL certificates
```

## Short description

TLS certificate reload requested.

## Trigger condition

Certificate-context reloading is invoked with the explicit reload flag.

## Explanation

Reloading begins for configured certificates; the message precedes validation and does not confirm success. The ordinary TLS and HTTP/3 caches can each emit it.

## Possible causes

- Certificate-context reloading is invoked with the explicit reload flag.

## What to check

1. Check subsequent certificate, key, and server-name diagnostics.
