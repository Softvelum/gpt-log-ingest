# Playlist content download did not become ready

Product: Nimble Streamer
Component: Server playlists
Logging function: log_e
Log level: error
Documentation ID: log_e-0eca4b2f650c9e1e

## Log pattern

```text
failed to download content url=%s
```

## Parameters

- Argument 1 — Content location: Text identifying a local media file or a remote cache key.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to download content url=/media/program.mp4
```

## Short description

Playlist content download did not become ready.

## Trigger condition

A notified cache item is no longer pending or downloading and is not ready.

## Explanation

The observer and request are removed without retaining the content.

## Possible causes

- A notified cache item is no longer pending or downloading and is not ready.

## What to check

1. Check the preceding content download and cache failure messages.

## Recommended action

Check the preceding content download and cache failure messages.
