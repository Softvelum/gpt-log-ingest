# WHEP helper media read failed

Product: Nimble Streamer
Component: WHEP media delivery
Logging function: log_d
Log level: debug
Documentation ID: log_d-c328f3ee32ba3db5

## Log pattern

```text
failed to read socket for whep session [%s]
```

## Parameters

- Argument 1 — Playback session URL: Relative WHEP session deletion URL including its session signature, text; identifies this playback session.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to read socket for whep session [/live/camera/whep.stream?whepsign=example-session]
```

## Short description

WHEP helper media read failed.

## Trigger condition

Reading helper messages returns end-of-connection, a hard socket error, or an impossible accumulated byte count larger than the input buffer.

## Explanation

The worker removes the session from active playback, marks it aborted, stops socket monitoring, and releases its helper media connection.

## Possible causes

- The helper closes its session socket or the socket read fails; would-block alone is accepted.

## What to check

1. Check the WHEP helper connection and nearby session shutdown events.

## Recommended action

Restore the helper or reconnect playback if the closure was unexpected.
