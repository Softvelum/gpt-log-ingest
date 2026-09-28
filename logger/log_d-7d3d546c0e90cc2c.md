# Panel connection through proxy started

Product: Nimble Streamer
Component: Panel synchronization and proxy transport
Logging function: log_d
Log level: debug
Documentation ID: log_d-7d3d546c0e90cc2c

## Log pattern

```text
connecting to '%s:%d' via proxy '%s' (%s:%d)
```

## Parameters

- Argument 1 — Host: Destination host or proxy host, text.
- Argument 2 — Port: TCP destination port number, integer.
- Argument 3 — Proxy host: Configured proxy hostname, text.
- Argument 4 — Proxy address: Selected resolved IPv4 proxy address, text.
- Argument 5 — Proxy port: Configured proxy TCP port number, integer.

## Log examples

Illustrative examples; these are not captured server logs.

```text
connecting to 'panel.example:443' via proxy 'proxy.example' (192.0.2.20:8080)
```

## Short description

Panel connection through proxy started.

## Trigger condition

A resolved proxy address is selected for a connection attempt while silent mode is disabled.

## Explanation

The TCP socket is created and connected to the proxy next; a tunnel to the original destination is requested afterward.
