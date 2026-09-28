# Playlist output stream cannot be separated

Product: Nimble Streamer
Component: Server playlists
Logging function: log_e
Log level: error
Documentation ID: log_e-137209cba176defe

## Log pattern

```text
[ServerPlaylistProcessor] task stream name is invalid [%s]
```

## Parameters

- Argument 1 — Playlist stream: Text naming the output application and stream.

## Log examples

Illustrative examples; these are not captured server logs.

```text
[ServerPlaylistProcessor] task stream name is invalid [live/channel]
```

## Short description

Playlist output stream cannot be separated.

## Trigger condition

An update reaches the output-name check but the name cannot be split into nonempty application and stream parts.

## Explanation

The playlist processor becomes failed and rejects the update. Earlier task validation normally prevents this path.

## Possible causes

- An update reaches the output-name check but the name cannot be split into nonempty application and stream parts.

## What to check

1. Use a name with nonempty application and stream separated by a slash.

## Recommended action

Use a name with nonempty application and stream separated by a slash.
