# Media request is waiting for cache progress

Product: Nimble Streamer
Component: HTTP client sessions
Logging function: log_d
Log level: debug
Documentation ID: log_d-db4aa48f88cae1b0

## Log pattern

```text
waiting for updates for cache item key='%s'
```

## Parameters

- Argument 1 — Cache key: Text identifying the requested media or playlist cache item.

## Log examples

Illustrative examples; these are not captured server logs.

```text
waiting for updates for cache item key='/live/camera/segment1.ts'
```

## Short description

Media request is waiting for cache progress.

## Trigger condition

A DVR, live HLS, remote VOD, advertising, or transmuxed response finds an item newly reserved or still receiving data after observer registration.

## Explanation

The request remains pending until a cache update makes the output ready or reports failure.

## Possible causes

- A DVR, live HLS, remote VOD, advertising, or transmuxed response finds an item newly reserved or still receiving data after observer registration.

## What to check

1. No action is needed for a short wait. If it persists, follow the matching producer, download, or transmuxing messages for this key.

## Recommended action

No action is needed for a short wait. If it persists, follow the matching producer, download, or transmuxing messages for this key.
