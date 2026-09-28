# Constant-rate sender queue limit is too small

Product: Nimble Streamer
Component: MPEG-TS transport input and output
Logging function: log_e
Log level: error
Documentation ID: log_e-cb9cdecdfe6948a8

## Log pattern

```text
cbr list size for [%s:%d] is too short,size=%lld
```

## Parameters

- Argument 1 — Destination: Configured destination IP address.
- Argument 2 — Port: Configured destination port.
- Argument 3 — Queue limit: Calculated maximum number of queued transport datagrams.

## Log examples

Illustrative examples; these are not captured server logs.

```text
cbr list size for [192.0.2.20:5000] is too short,size=4
```

## Short description

Constant-rate sender queue limit is too small.

## Trigger condition

The calculated maximum send-queue size is below ten transport datagrams.

## Explanation

Sender startup returns failure before creating sender and muxer worker threads.

## Possible causes

- The calculated maximum send-queue size is below ten transport datagrams.

## What to check

1. Check configured mux rate and maximum CBR buffer duration.

## Recommended action

Increase the configured buffer/rate combination to allow at least ten queued transport datagrams.
