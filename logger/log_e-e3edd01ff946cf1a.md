# Cannot enable SRT rendezvous mode

Product: Nimble Streamer
Component: SRT connections and session reporting
Logging function: log_e
Log level: error
Documentation ID: log_e-e3edd01ff946cf1a

## Log pattern

```text
failed to set rendezvous mode s=%d
```

## Parameters

- Argument 1 — Socket: SRT socket identifier.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to set rendezvous mode s=100
```

## Short description

Cannot enable SRT rendezvous mode.

## Trigger condition

The library rejects enabling rendezvous mode on an ingest or output socket.

## Explanation

Connection setup stops before local rendezvous binding or peer connection. The output path includes regular and non-Windows constant-bitrate senders.

## Possible causes

- The library rejects enabling rendezvous mode on an ingest or output socket.

## What to check

1. Check the SRT socket settings and intended connection mode.
