# WebRTC helper welcome length invalid

Product: Nimble Streamer
Component: WebRTC helper connection
Logging function: log_d
Log level: debug
Documentation ID: log_d-c1e2dd3cd92aee98

## Log pattern

```text
wrong welcome message size. fd=%d, size=%d
```

## Parameters

- Argument 1 — Socket: Socket descriptor, integer; no unit.
- Argument 2 — Welcome size: Declared welcome-message body length, bytes, decoded as an unsigned 32-bit value and cast to int for display.

Shared by the WHIP publishing and WHEP playback helper listeners; both enforce the same welcome-size bound.

## Log examples

Illustrative examples; these are not captured server logs.

```text
wrong welcome message size. fd=12, size=65537
```

## Short description

WebRTC helper welcome length invalid.

## Trigger condition

A newly accepted WHIP or WHEP helper connection declares a welcome-message body length of zero or more than 65,536 bytes.

## Explanation

The socket is removed from monitoring and the accepted connection is discarded before its welcome message is parsed.

## Possible causes

- The helper sends a malformed or incompatible length-prefixed welcome message.

## What to check

1. Check that the correct WebRTC helper connects to the configured control listener and uses the matching message protocol.

## Recommended action

Correct the helper/protocol mismatch and reconnect.
