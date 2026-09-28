# WHEP helper session message rejected

Product: Nimble Streamer
Component: WHEP media delivery
Logging function: log_d
Log level: debug
Documentation ID: log_d-1d2ece839f184cfc

## Log pattern

```text
wrong message for whep session [%s]
```

## Parameters

- Argument 1 — Playback session URL: Relative WHEP session deletion URL including its session signature, text; identifies this playback session.

Valid JSON with an unrecognized message type is otherwise ignored; this message does not indicate strict validation of every JSON field.

## Log examples

Illustrative examples; these are not captured server logs.

```text
wrong message for whep session [/live/camera/whep.stream?whepsign=example-session]
```

## Short description

WHEP helper session message rejected.

## Trigger condition

A complete helper message cannot be parsed as JSON, or its adaptive-bitrate action returns failure.

## Explanation

The worker removes the session from active playback, marks it aborted, stops socket monitoring, and releases its helper media connection.

## Possible causes

- The helper sends malformed JSON or an unsupported adaptive-bitrate action.
- A higher/lower bitrate request cannot locate its live adaptive-bitrate stream definition.

## What to check

1. Check helper messages and whether the playback adaptive-bitrate stream remains available.

## Recommended action

Correct the helper command or restore the stream definition and reconnect.
