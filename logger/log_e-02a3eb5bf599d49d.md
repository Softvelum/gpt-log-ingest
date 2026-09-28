# RTSP transport negotiation has no permitted UDP fallback

Product: Nimble Streamer
Component: RTSP connections and control
Logging function: log_e
Log level: error
Documentation ID: log_e-02a3eb5bf599d49d

## Log pattern

```text
failed to set UDP transport, socket=%d
```

## Parameters

- Argument 1 — Socket: Integer operating-system socket descriptor identifying this connection.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to set UDP transport, socket=25
```

## Short description

RTSP transport negotiation has no permitted UDP fallback.

## Trigger condition

An interleaved TCP SETUP receives status 461 and the configured transport policy does not permit TCP-to-UDP fallback.

## Explanation

The client shuts down. Despite the message naming UDP, the rejected request used interleaved TCP; the missing option is permission to try UDP next.

## Possible causes

- An interleaved TCP SETUP receives status 461 and the configured transport policy does not permit TCP-to-UDP fallback.

## What to check

1. Check the source transport support and the configured fallback order.

## Recommended action

Use a mutually supported transport or permit the appropriate fallback.

## Related log messages

- [RTSP transport negotiation has no permitted TCP fallback](log_e-e1d611dbf75616bd.md)
