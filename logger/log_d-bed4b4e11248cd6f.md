# SRT publisher connection accepted

Product: Nimble Streamer
Component: SRT publishing authentication
Logging function: log_d
Log level: debug
Documentation ID: log_d-bed4b4e11248cd6f

## Log pattern

```text
srt connection accepted for [%s:%d], socket=%d
```

## Parameters

- Argument 1 — Address: Configured endpoint address.
- Argument 2 — Port: Configured endpoint port number.
- Argument 3 — Socket: SRT socket identifier.

## Log examples

Illustrative examples; these are not captured server logs.

```text
srt connection accepted for [0.0.0.0:9000], socket=101
```

## Short description

SRT publisher connection accepted.

## Trigger condition

The accepted publisher socket is configured nonblocking and registered for input/error events.

## Explanation

Camera creation and any stream-ID authorization still follow. This message does not by itself confirm successful publishing.

## Possible causes

- The accepted publisher socket is configured nonblocking and registered for input/error events.

## What to check

1. Check the following stream-ID and authorization messages.
