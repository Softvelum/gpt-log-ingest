# Local source or sink list received

Product: Nimble Streamer
Component: Local media source discovery
Logging function: log_d
Log level: debug
Documentation ID: log_d-a9ac3cf1cad46209

## Log pattern

```text
local network list received, json={%s}
```

## Parameters

- Argument 1 — List payload: Received payload interpreted as text; normally a JSON source or sink list. The format adds an extra pair of braces around it.

## Log examples

Illustrative examples; these are not captured server logs.

```text
local network list received, json={["camera"]}
```

## Short description

Local source or sink list received.

## Trigger condition

The local helper sends a source-list or sink-list message.

## Explanation

Its complete payload is saved, the received flag is set, and the message handler reports completion.
