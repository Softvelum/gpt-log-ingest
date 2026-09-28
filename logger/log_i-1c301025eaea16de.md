# Playlist content reference released

Product: Nimble Streamer
Component: Server playlists
Logging function: log_i
Log level: info
Documentation ID: log_i-1c301025eaea16de

## Log pattern

```text
deleted content url=%s
```

## Parameters

- Argument 1 — Content location: Text identifying a local media file or a remote cache key.

## Log examples

Illustrative examples; these are not captured server logs.

```text
deleted content url=/media/program.mp4
```

## Short description

Playlist content reference released.

## Trigger condition

A synchronized task set no longer retains a previously held content item.

## Explanation

The playlist cache reference is removed; this does not itself delete an origin file.

## Possible causes

- A synchronized task set no longer retains a previously held content item.
