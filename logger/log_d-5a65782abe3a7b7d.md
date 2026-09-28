# Listener socket created

Product: Nimble Streamer
Component: Server startup and services
Logging function: log_d
Log level: debug
Documentation ID: log_d-5a65782abe3a7b7d

## Log pattern

```text
opened listener socket s=%d (%s) port=%d
```

## Parameters

- Argument 1 — Socket: Integer newly created socket descriptor.
- Argument 2 — Security mode: Either SSL or plain, according to the configured port list.
- Argument 3 — Port: Configured integer port.

## Log examples

Illustrative examples; these are not captured server logs.

```text
opened listener socket s=12 (SSL) port=443
```

## Short description

Listener socket created.

## Trigger condition

Creating an IPv4 or IPv6 socket for one configured interface and port succeeds.

## Explanation

The socket is recorded before address conversion, binding, nonblocking setup and any TCP listen operation. This pattern also covers HTTP/3 UDP sockets, so creation alone does not prove an active TCP listener.
