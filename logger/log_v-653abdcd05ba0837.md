# HTTP/2 unknown frame skipped

Product: Nimble Streamer
Component: HTTP/2 connection
Logging function: log_v
Log level: verbose
Documentation ID: log_v-653abdcd05ba0837

## Log pattern

```text
skip unknown h2 frame type by s=%d, t=%d
```

## Parameters

- Argument 1 — Socket: Socket descriptor, integer; no unit.
- Argument 2 — Frame type: Received frame type code, unsigned byte converted to integer and displayed in decimal.

## Log examples

Illustrative examples; these are not captured server logs.

```text
skip unknown h2 frame type by s=12, t=15
```

## Short description

HTTP/2 unknown frame skipped.

## Trigger condition

A complete incoming HTTP/2 frame has a type outside the handled frame types.

## Explanation

Its full header and payload are consumed, its frame state is cleared, and command processing continues.
