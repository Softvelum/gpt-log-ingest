# TLS session could not be bound to the accepted socket

Product: Nimble Streamer
Component: Client and origin sockets
Logging function: log_e
Log level: error
Documentation ID: log_e-a43740d3ef09ccdc

## Log pattern

```text
SSL_set_fd failed for ssl_=%p %s s=%d
```

## Parameters

- Argument 1 — TLS session: TLS session address used for diagnostic correlation.
- Argument 2 — Peer role: Text name identifying the socket role, such as client or origin.
- Argument 3 — Socket: Integer socket descriptor.

## Log examples

Illustrative examples; these are not captured server logs.

```text
SSL_set_fd failed for ssl_=0x1000 client s=12
```

## Short description

TLS session could not be bound to the accepted socket.

## Trigger condition

Associating the accepted socket descriptor with the TLS session fails.

## Explanation

TLS resources are released and accepted-connection TLS initialization returns failure.

## Possible causes

- The TLS library rejected socket association.

## What to check

1. Inspect the TLS session and accepted socket descriptor.
