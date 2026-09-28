# SRT output stopped by pay-per-view denial

Product: Nimble Streamer
Component: SRT connections and session reporting
Logging function: log_d
Log level: debug
Documentation ID: log_d-c1dbaacf49de88c9

## Log pattern

```text
client_ip=%s host=%s ppv_id='%s' is denied by PPV
```

## Parameters

- Argument 1 — Peer: Receiver IP address.
- Argument 2 — Host: Local reporting endpoint address and port.
- Argument 3 — Client ID: Pay-per-view client identifier.

## Log examples

Illustrative examples; these are not captured server logs.

```text
client_ip=192.0.2.20 host=192.0.2.1:9000 ppv_id='example-client' is denied by PPV
```

## Short description

SRT output stopped by pay-per-view denial.

## Trigger condition

The sender observes that its registered session was denied by pay-per-view access control.

## Explanation

Sending returns failure before writing additional output bytes.

## Possible causes

- The sender observes that its registered session was denied by pay-per-view access control.

## What to check

1. Check the indicated pay-per-view client and access decision.
