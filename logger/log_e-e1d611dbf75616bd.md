# RTSP transport negotiation has no permitted TCP fallback

Product: Nimble Streamer
Component: RTSP connections and control
Logging function: log_e
Log level: error
Documentation ID: log_e-e1d611dbf75616bd

## Log pattern

```text
failed to set TCP transport, socket=%d
```

## Parameters

- Argument 1 — Socket: Integer operating-system socket descriptor identifying this connection.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to set TCP transport, socket=25
```

## Short description

RTSP transport negotiation has no permitted TCP fallback.

## Trigger condition

A UDP SETUP receives status 461 and the configured transport policy does not permit UDP-to-TCP fallback.

## Explanation

The client shuts down. The rejected request used UDP; the message names the unavailable TCP fallback.

## Possible causes

- A UDP SETUP receives status 461 and the configured transport policy does not permit UDP-to-TCP fallback.

## What to check

1. Check the source transport support and the configured fallback order.

## Recommended action

Use a mutually supported transport or permit the appropriate fallback.

## Related log messages

- [RTSP transport negotiation has no permitted UDP fallback](log_e-02a3eb5bf599d49d.md)
