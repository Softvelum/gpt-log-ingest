# Playlist task is invalid

Product: Nimble Streamer
Component: Server playlists
Logging function: log_e
Log level: error
Documentation ID: log_e-05e863fe8b795cec

## Log pattern

```text
invalid Task stream=[%s]. Ignored
```

## Parameters

- Argument 1 — Playlist stream: Text naming the output application and stream.

## Log examples

Illustrative examples; these are not captured server logs.

```text
invalid Task stream=[live/channel]. Ignored
```

## Short description

Playlist task is invalid.

## Trigger condition

A task with a nonempty stream name fails validation of its application/stream name or inactivity timeout.

## Explanation

The task is excluded from the synchronized playlist set.

## Possible causes

- A task with a nonempty stream name fails validation of its application/stream name or inactivity timeout.

## What to check

1. Use a nonempty application/stream name and a nonnegative inactivity timeout.

## Recommended action

Use a nonempty application/stream name and a nonnegative inactivity timeout.
