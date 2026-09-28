# Server playlist worker stopped

Product: Nimble Streamer
Component: Server playlists
Logging function: log_d
Log level: debug
Documentation ID: log_d-fa8ae70ec8a9cc9a

## Log pattern

```text
[%s] server playlist stopped
```

## Parameters

- Argument 1 — Playlist stream: Text naming the output application and stream.

## Log examples

Illustrative examples; these are not captured server logs.

```text
[live/channel] server playlist stopped
```

## Short description

Server playlist worker stopped.

## Trigger condition

A stop request or failed task update ends the playlist worker loop.

## Explanation

The worker marks itself finished so synchronization can remove it.

## Possible causes

- A stop request or failed task update ends the playlist worker loop.

## What to check

1. Check preceding task-update errors if the stop was unexpected.

## Recommended action

Check preceding task-update errors if the stop was unexpected.

## Related log messages

- [Server playlist worker started](log_d-8c489517c3bb6d50.md)
