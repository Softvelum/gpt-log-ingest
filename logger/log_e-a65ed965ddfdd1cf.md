# SRT ingest receive failed or connection closed

Product: Nimble Streamer
Component: SRT connections and session reporting
Logging function: log_e
Log level: error
Documentation ID: log_e-a65ed965ddfdd1cf

## Log pattern

```text
connection closed for [%s:%d] socket=%d errno=%d srterror=[%s]
```

## Parameters

- Argument 1 — Address: Configured endpoint address.
- Argument 2 — Port: Configured endpoint port number.
- Argument 3 — Socket: SRT socket identifier.
- Argument 4 — Error number: SRT error code, despite the errno label.
- Argument 5 — Error: SRT library error description.

## Log examples

Illustrative examples; these are not captured server logs.

```text
connection closed for [192.0.2.10:9000] socket=100 errno=2001 srterror=[Connection was broken]
```

## Short description

SRT ingest receive failed or connection closed.

## Trigger condition

Receiving an SRT message returns no positive payload and the error is not the ordinary asynchronous receive-wait condition.

## Explanation

The camera reports failure; its owner can remove it or try a configured redundant source.

## Possible causes

- Receiving an SRT message returns no positive payload and the error is not the ordinary asynchronous receive-wait condition.

## What to check

1. Check the SRT error and the source connection state.
