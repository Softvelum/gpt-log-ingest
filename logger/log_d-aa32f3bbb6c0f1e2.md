# Observed cache item changed state

Product: Nimble Streamer
Component: HTTP client sessions
Logging function: log_d
Log level: debug
Documentation ID: log_d-aa32f3bbb6c0f1e2

## Log pattern

```text
[CliS::OnCIU] key='%s' st=%d (%p st=%d s=%d)
```

## Parameters

- Argument 1 — Cache key: Text identifying the requested media or playlist cache item.
- Argument 2 — Cache state: Integer cache state: 0 newly reserved, 1 receiving data, 2 fragmented, 3 ready, or 4 marked for removal.
- Argument 3 — Session address: Pointer identifying this client session within the process.
- Argument 4 — Session state: Integer identifying the HTTP client session phase: 0 reading a request, 1 sending a response, 2 connecting to origin, 3 sending to origin, 4 reading origin headers, 5 reading an origin playlist, 6 reading origin media, 29 shutting down, or 30 finished; intermediate values identify pending asynchronous work.
- Argument 5 — Client socket: Integer identifying the local client socket.

## Log examples

Illustrative examples; these are not captured server logs.

```text
[CliS::OnCIU] key='/live/camera/segment1.ts' st=1 (0x1234 st=4 s=18)
```

## Short description

Observed cache item changed state.

## Trigger condition

A cache update matches the session current item or completes its required-item wait.

## Explanation

The item state is recorded before handling cancellation, body availability, playlists, or fragments.

## Possible causes

- A cache update matches the session current item or completes its required-item wait.

## What to check

1. Not applicable; this is normal diagnostic progress. Use the socket identifier to follow the same request if troubleshooting.

## Recommended action

Not applicable; this is normal diagnostic progress. Use the socket identifier to follow the same request if troubleshooting.
