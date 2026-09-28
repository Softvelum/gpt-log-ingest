# Playlist block is invalid

Product: Nimble Streamer
Component: Server playlists
Logging function: log_e
Log level: error
Documentation ID: log_e-217b68e4ab416abd

## Log pattern

```text
invalid Block id=[%s]. Ignored
```

## Parameters

- Argument 1 — Block ID: Text identifying the schedule block.

## Log examples

Illustrative examples; these are not captured server logs.

```text
invalid Block id=[morning]. Ignored
```

## Short description

Playlist block is invalid.

## Trigger condition

A block lacks an identifier or valid streams, or has negative duration or repeat settings.

## Explanation

The block is omitted from the task.

## Possible causes

- A block lacks an identifier or valid streams, or has negative duration or repeat settings.

## What to check

1. Supply a block identifier, at least one valid stream, and nonnegative duration and repeat settings.

## Recommended action

Supply a block identifier, at least one valid stream, and nonnegative duration and repeat settings.
