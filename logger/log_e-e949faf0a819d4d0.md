# RTSP input transport selection is invalid

Product: Nimble Streamer
Component: RTSP connections and control
Logging function: log_e
Log level: error
Documentation ID: log_e-e949faf0a819d4d0

## Log pattern

```text
unsupported transport=%d, socket=%d
```

## Parameters

- Argument 1 — Transport value: Integer internal transport selection; the numeric value is dimensionless.
- Argument 2 — Socket: Integer operating-system socket descriptor identifying this connection.

## Log examples

Illustrative examples; these are not captured server logs.

```text
unsupported transport=99, socket=25
```

## Short description

RTSP input transport selection is invalid.

## Trigger condition

The configured transport value matches none of the supported TCP, UDP, ordered fallback, or HTTP-tunnel choices.

## Explanation

Track setup returns failure without constructing a transport request.

## Possible causes

- The configured transport value matches none of the supported TCP, UDP, ordered fallback, or HTTP-tunnel choices.

## What to check

1. Check the selected RTSP transport mode.

## Recommended action

Select a supported RTSP transport mode.
