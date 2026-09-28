# SRT receiver session registration rejected

Product: Nimble Streamer
Component: SRT connections and session reporting
Logging function: log_v
Log level: verbose
Documentation ID: log_v-7ed42e7c1ecb4628

## Log pattern

```text
RegisterSRTSession failed for s=%d (%s)
```

## Parameters

- Argument 1 — Socket: SRT socket identifier.
- Argument 2 — Peer: Receiver IP address captured during registration.

## Log examples

Illustrative examples; these are not captured server logs.

```text
RegisterSRTSession failed for s=101 (192.0.2.20)
```

## Short description

SRT receiver session registration rejected.

## Trigger condition

A receiver accepted by a dedicated output listener fails the session-registration step.

## Explanation

The accepted socket is closed while the listening socket remains available.

## Possible causes

- A receiver accepted by a dedicated output listener fails the session-registration step.

## What to check

1. Check preceding reporting, licensing, address, and access-control diagnostics.

## Related log messages

- [SRT reporting cannot identify local address](log_e-4045659fb594f301.md)
- [SRT reporting cannot identify receiver address](log_e-6455929704407784.md)
- [SRT session reporting requires Addenda](log_e-b002ebc69defb7c2.md)
