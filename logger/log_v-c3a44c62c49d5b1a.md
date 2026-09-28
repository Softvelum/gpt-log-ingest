# Constant-rate sending worker has stopped

Product: Nimble Streamer
Component: MPEG-TS transport input and output
Logging function: log_v
Log level: verbose
Documentation ID: log_v-c3a44c62c49d5b1a

## Log pattern

```text
cbr worker thread stopped s=%d '%s:%d'
```

## Parameters

- Argument 1 — Socket: Numeric socket descriptor.
- Argument 2 — Destination: Configured destination IP address.
- Argument 3 — Port: Configured destination port.

## Log examples

Illustrative examples; these are not captured server logs.

```text
cbr worker thread stopped s=42 '192.0.2.20:5000'
```

## Short description

Constant-rate sending worker has stopped.

## Trigger condition

The muxing side tries to send another buffer after the CBR sending worker has marked itself stopped.

## Explanation

Buffer submission returns failure and the sending loop transitions to shutdown.

## Possible causes

- The muxing side tries to send another buffer after the CBR sending worker has marked itself stopped.

## What to check

1. Check preceding CBR send errors or shutdown activity.

## Recommended action

Resolve any preceding send failure before restoring the output.
