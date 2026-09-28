# Transport socket or SRT group is created

Product: Nimble Streamer
Component: MPEG-TS transport input and output
Logging function: log_d
Log level: debug
Documentation ID: log_d-bddafbb1a6d0911a

## Log pattern

```text
socket s %d
```

## Parameters

- Argument 1 — Socket or group: Newly created UDP descriptor, SRT socket handle, or SRT group handle.

## Log examples

Illustrative examples; these are not captured server logs.

```text
socket s 42
```

## Short description

Transport socket or SRT group is created.

## Trigger condition

Creating a UDP sender socket, SRT socket, or SRT bonding group succeeds.

## Explanation

Further socket/group setup continues and can still fail. The numeric value is a UDP descriptor or an SRT handle depending on the path.

## Possible causes

- Creating a UDP sender socket, SRT socket, or SRT bonding group succeeds.

## What to check

1. Correlate the handle with subsequent setup and connection messages.

## Recommended action

No action is needed for normal creation.
