# Socket event has no registered owner

Product: Nimble Streamer
Component: Live inputs, publishers, senders, and DVR management
Logging function: log_e
Log level: error
Documentation ID: log_e-746fe1df01aabc1b

## Log pattern

```text
unknown socket event: [%d]:%d
```

## Parameters

- Argument 1 — socket: Descriptor delivered with the event; it may be an SRT descriptor in SRT paths.
- Argument 2 — event\_mask: Received event flags printed as a decimal integer; the event source determines the bit meanings.

## Log examples

Illustrative examples; these are not captured server logs.

```text
unknown socket event: [42]:1
```

## Short description

Socket event has no registered owner.

## Trigger condition

An event arrives for a descriptor absent from the relevant active-session or sender table. DVR management instead reports it when no current request processor exists.

## Explanation

Most occurrences log and return without closing the descriptor or stopping the thread. SRT listening input additionally removes the unknown descriptor from its selector. The SRT constant-rate sender branch reaches the end of its handler after logging. Listener-based variants first rule out a recognized listening socket. The same text is used for RTMP, RTSP, Icecast, MPEG-TS transport senders, SRT, Zixi, generic media senders, live pulls, and DVR management; protocol-specific occurrences require the corresponding build support.

## Possible causes

- The descriptor no longer has a matching registered object, or DVR management has no active processor.

## What to check

1. Check earlier creation, removal, and shutdown messages for the same descriptor and component.
