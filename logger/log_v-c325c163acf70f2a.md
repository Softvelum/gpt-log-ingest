# External request connection succeeded

Product: Nimble Streamer
Component: External HTTP requests
Logging function: log_v
Log level: verbose
Documentation ID: log_v-c325c163acf70f2a

## Log pattern

```text
connected to %s s=%d
```

## Parameters

- Argument 1 — peer: Peer role label.
- Argument 2 — socket: Peer socket descriptor.

## Log examples

Illustrative examples; these are not captured server logs.

```text
connected to peer s=12
```

## Short description

External request connection succeeded.

## Trigger condition

The connection socket-error query succeeds and reports no error.

## Explanation

The request is prepared next; successful connection does not prove the HTTP exchange will succeed.
