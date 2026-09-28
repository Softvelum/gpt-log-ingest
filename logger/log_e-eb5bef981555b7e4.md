# Playlist activity log cannot be written

Product: Nimble Streamer
Component: Server playlists
Logging function: log_e
Log level: error
Documentation ID: log_e-eb5bef981555b7e4

## Log pattern

```text
failed to write server playlist log file='%s'
```

## Parameters

- Argument 1 — Activity log file: Text naming the destination activity log.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to write server playlist log file='/var/log/playlist.log'
```

## Short description

Playlist activity log cannot be written.

## Trigger condition

Appending a playlist start, stop, block, or stream-change event to its configured activity log fails.

## Explanation

The event is not successfully appended; this routine does not stop playback.

## Possible causes

- Appending a playlist start, stop, block, or stream-change event to its configured activity log fails.

## What to check

1. Restore write access and free space for the configured activity log.

## Recommended action

Restore write access and free space for the configured activity log.
