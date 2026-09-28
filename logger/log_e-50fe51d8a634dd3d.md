# AJA writer cannot connect to its media-session endpoint

Product: Nimble Streamer
Component: AJA capture and playback
Logging function: log_e
Log level: error
Documentation ID: log_e-50fe51d8a634dd3d

## Log pattern

```text
failed to connect to AJA writer id=%llu name='%s' endpoint=%s:%d
```

## Parameters

- Argument 1 — Session ID: Positive unsigned 64-bit identifier for the reader or writer session.
- Argument 2 — Session name: Configured AJA device and channel name.
- Argument 3 — Endpoint address: IP address of the internal media-session connection.
- Argument 4 — Endpoint port: Decimal TCP port of the internal media-session connection.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to connect to AJA writer id=1 name='0/0' endpoint=127.0.0.1:9000
```

## Short description

AJA writer cannot connect to its media-session endpoint.

## Trigger condition

Connecting the worker to the configured internal IP address and port returns failure.

## Explanation

The worker performs cleanup and marks itself finished before entering its event loop. This endpoint is the internal media connection, not the SDI connector.

## Possible causes

- Connecting the worker to the configured internal IP address and port returns failure.

## What to check

1. Check that the internal endpoint is listening at the supplied address and port.
