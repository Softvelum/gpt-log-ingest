# SLDP stream status is prepared

Product: Nimble Streamer
Component: SLDP playback and advertising
Logging function: log_d
Log level: debug
Documentation ID: log_d-43035ecfb1aef575

## Log pattern

```text
s=%d, send status: %s
```

## Parameters

- Argument 1 — Socket: Connection descriptor reported by the playback transport.
- Argument 2 — Status JSON: Complete generated status response; optional steady and system times are decimal microsecond strings.

## Log examples

Illustrative examples; these are not captured server logs.

```text
s=42, send status: {"command":"status","info":[{"stream":"camera","stream_info":{"bandwidth":"0"}}]}
```

## Short description

SLDP stream status is prepared.

## Trigger condition

The initial status response is constructed while the outgoing buffer is empty.

## Explanation

The response lists available streams, codec/timescale information, and applicable adaptive-stream settings. It is logged before framing and sending, so the message does not prove delivery.

## Possible causes

- The initial status response is constructed while the outgoing buffer is empty.

## What to check

1. Check the announced stream list against the requested playback URL.

## Recommended action

No action is required for an expected status exchange.
