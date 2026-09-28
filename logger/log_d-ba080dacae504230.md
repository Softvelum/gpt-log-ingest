# WHEP helper message length invalid

Product: Nimble Streamer
Component: WHEP media delivery
Logging function: log_d
Log level: debug
Documentation ID: log_d-ba080dacae504230

## Log pattern

```text
wrong message size for whep session [%s]
```

## Parameters

- Argument 1 — Playback session URL: Relative WHEP session deletion URL including its session signature, text; identifies this playback session.

## Log examples

Illustrative examples; these are not captured server logs.

```text
wrong message size for whep session [/live/camera/whep.stream?whepsign=example-session]
```

## Short description

WHEP helper message length invalid.

## Trigger condition

The buffered message header declares a body length of zero or at least 16,380 bytes.

## Explanation

The worker removes the session from active playback, marks it aborted, stops socket monitoring, and releases its helper media connection.

## Possible causes

- The helper sends a malformed or oversized length-prefixed session message.

## What to check

1. Check the helper message framing and body size; the accepted body length is 1 through 16,379 bytes.

## Recommended action

Correct the helper message format or size and reconnect playback.
