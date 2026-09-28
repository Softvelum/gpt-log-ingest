# Socket event cannot continue normal processing

Product: Nimble Streamer
Component: Socket event handling
Logging function: log_d
Log level: debug
Documentation ID: log_d-eda3afbbce3e6a94

## Log pattern

```text
event for fd=%d error event=0x%x
```

## Parameters

- Argument 1 — Socket: Socket descriptor, integer; no unit.
- Argument 2 — Event mask: Socket event flags, integer printed in hexadecimal; bits represent input, output, error, peer-close and hangup indications. This is not an operating-system error code.

A pending WebRTC read failure can print an input-ready mask because the read result, rather than the mask alone, caused this message.

## Log examples

Illustrative examples; these are not captured server logs.

```text
event for fd=12 error event=0x8
```

## Short description

Socket event cannot continue normal processing.

## Trigger condition

A listener receives an error or lacks an input-ready event; a process session reaches an error, peer-close or hangup event; a pending WebRTC welcome connection receives an event other than input alone, closes, or has a nonretryable read error; an active WHEP session receives a connection error; or a pending Whisper connection receives an error event.

## Explanation

Listeners skip this event without accepting or reading. Generic process workers mark the attempt failed; local discovery and output helpers stop their connection. Pending scenario, WebRTC and Whisper connections are removed and closed. Active WHEP sessions are stopped and removed.

## Possible causes

- The peer closed the connection, the event monitor reported a socket error, the event was inappropriate for that connection state, or a pending WebRTC read failed.

## What to check

1. Use the descriptor, event mask and emitting worker to distinguish listener events from session failures; check nearby peer and socket errors.

## Recommended action

No action is needed for an expected peer shutdown. For repeated failures, restore the affected local peer or listener operation.
