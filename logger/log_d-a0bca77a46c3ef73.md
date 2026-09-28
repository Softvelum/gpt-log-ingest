# Transport input reports an available elementary stream

Product: Nimble Streamer
Component: MPEG-TS stream registration and parsing
Logging function: log_d
Log level: debug
Documentation ID: log_d-a0bca77a46c3ef73

## Log pattern

```text
strm pid=%d type=%d
```

## Parameters

- Argument 1 — PID: Available elementary-stream packet identifier.
- Argument 2 — Type: Available stream type code in decimal, including recognized descriptor-derived types.

## Log examples

Illustrative examples; these are not captured server logs.

```text
strm pid=256 type=27
```

## Short description

Transport input reports an available elementary stream.

## Trigger condition

The stream-selection update visits one entry in the available PID/type map.

## Explanation

The stream type is considered for video, audio, splice, subtitle, or metadata selection. Logging does not establish that this PID was ultimately selected.

## Possible causes

- The stream-selection update visits one entry in the available PID/type map.

## What to check

1. Compare the PID and decimal type with the input program map and selected track settings.

## Recommended action

No action is needed for an expected elementary stream.
