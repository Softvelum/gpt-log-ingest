# Constant-rate sender settings require a restart

Product: Nimble Streamer
Component: MPEG-TS transport input and output
Logging function: log_v
Log level: verbose
Documentation ID: log_v-bf27b6feddd78b0c

## Log pattern

```text
udp cbr parameters changed. Restarting cbr sender s=%d '%s:%d'
```

## Parameters

- Argument 1 — Socket: Numeric socket descriptor.
- Argument 2 — Destination: Configured destination IP address.
- Argument 3 — Port: Configured destination port.

## Log examples

Illustrative examples; these are not captured server logs.

```text
udp cbr parameters changed. Restarting cbr sender s=42 '192.0.2.20:5000'
```

## Short description

Constant-rate sender settings require a restart.

## Trigger condition

Mux rate, maximum/start buffering, destination address, or destination port differs from the running CBR sender settings.

## Explanation

The settings update returns failure so its caller can replace the sender. Printed address and port are the old settings.

## Possible causes

- Mux rate, maximum/start buffering, destination address, or destination port differs from the running CBR sender settings.

## What to check

1. Compare old and new CBR settings.

## Recommended action

No action is needed when these changes are intentional.
