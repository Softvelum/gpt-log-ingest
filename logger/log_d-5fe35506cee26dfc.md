# SRT ingest connection attempt starting

Product: Nimble Streamer
Component: SRT connections and session reporting
Logging function: log_d
Log level: debug
Documentation ID: log_d-5fe35506cee26dfc

## Log pattern

```text
connecting srt s=%d to [%s:%hu]
```

## Parameters

- Argument 1 — Socket: SRT socket identifier.
- Argument 2 — Address: Configured endpoint address.
- Argument 3 — Port: Configured endpoint port number.

## Log examples

Illustrative examples; these are not captured server logs.

```text
connecting srt s=100 to [192.0.2.10:9000]
```

## Short description

SRT ingest connection attempt starting.

## Trigger condition

The receive socket and any rendezvous binding are ready, immediately before the peer-connect call.

## Explanation

The log records an attempt, before connection success is established.

## Possible causes

- The receive socket and any rendezvous binding are ready, immediately before the peer-connect call.

## What to check

1. Confirm the remote endpoint and subsequent connection result.
