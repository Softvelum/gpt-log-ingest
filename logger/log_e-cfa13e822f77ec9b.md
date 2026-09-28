# Playlist block response is missing

Product: Nimble Streamer
Component: Server playlists
Logging function: log_e
Log level: error
Documentation ID: log_e-cfa13e822f77ec9b

## Log pattern

```text
failed to receive server playlist block url='%s'
```

## Parameters

- Argument 1 — Request URL: Text identifying the playlist or block request.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to receive server playlist block url='https://api.example/playlist'
```

## Short description

Playlist block response is missing.

## Trigger condition

Block download processing finishes without a stored response for a task block location.

## Explanation

No external blocks are added to that task from this location.

## Possible causes

- Block download processing finishes without a stored response for a task block location.

## What to check

1. Check earlier failures for the block-definition request.

## Recommended action

Check earlier failures for the block-definition request.
