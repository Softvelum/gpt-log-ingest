# Client output paused until more body data arrives

Product: Nimble Streamer
Component: HTTP client sessions
Logging function: log_d
Log level: debug
Documentation ID: log_d-bccb35b3094dc89f

## Log pattern

```text
[CliS::AResp] no data for s=%d
```

## Parameters

- Argument 1 — Client socket: Integer identifying the local client socket.

## Log examples

Illustrative examples; these are not captured server logs.

```text
[CliS::AResp] no data for s=18
```

## Short description

Client output paused until more body data arrives.

## Trigger condition

All available body bytes are sent, or cached bytes have not yet reached the requested range offset.

## Explanation

Writable-event monitoring is disabled while disconnect monitoring remains enabled.

## Possible causes

- All available body bytes are sent, or cached bytes have not yet reached the requested range offset.

## What to check

1. Not applicable; this is normal diagnostic progress. Use the socket identifier to follow the same request if troubleshooting.

## Recommended action

Not applicable; this is normal diagnostic progress. Use the socket identifier to follow the same request if troubleshooting.
