# Playlist downloaded content cannot be acquired

Product: Nimble Streamer
Component: Server playlists
Logging function: log_e
Log level: error
Documentation ID: log_e-c89a9de04ec21a96

## Log pattern

```text
failed to request downloaded content url=%s
```

## Parameters

- Argument 1 — Content location: Text identifying a local media file or a remote cache key.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to request downloaded content url=/media/program.mp4
```

## Short description

Playlist downloaded content cannot be acquired.

## Trigger condition

A pending content notification arrives but the corresponding cache item cannot be obtained.

## Explanation

The observer and pending request are removed; the content is not retained.

## Possible causes

- A pending content notification arrives but the corresponding cache item cannot be obtained.

## What to check

1. Check preceding cache removal or download failures.

## Recommended action

Check preceding cache removal or download failures.
