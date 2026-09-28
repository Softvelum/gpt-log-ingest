# Playlist HTTP request failed

Product: Nimble Streamer
Component: Server playlists
Logging function: log_e
Log level: error
Documentation ID: log_e-aa39f4581ca3c85b

## Log pattern

```text
server playlist request failed url='%s' status='%s'
```

## Parameters

- Argument 1 — Request URL: Text identifying the playlist or block request.
- Argument 2 — HTTP status text: HTTP status written as text, or unknown when no parser is available.

## Log examples

Illustrative examples; these are not captured server logs.

```text
server playlist request failed url='https://api.example/playlist' status='503'
```

## Short description

Playlist HTTP request failed.

## Trigger condition

A playlist or block HTTP request invokes failure handling.

## Explanation

A block is removed from pending requests; a main request clears the response and marks synchronization failed.

## Possible causes

- A playlist or block HTTP request invokes failure handling.

## What to check

1. Check the destination and reported HTTP status.

## Recommended action

Check the destination and reported HTTP status.

## Related log messages

- [Failure reported for an unexpected playlist URL](log_e-90dc00174d0108bc.md)
