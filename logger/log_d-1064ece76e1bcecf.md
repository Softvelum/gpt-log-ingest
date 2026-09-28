# Pending media output canceled after DRM key failure

Product: Nimble Streamer
Component: VOD transmuxing
Logging function: log_d
Log level: debug
Documentation ID: log_d-1064ece76e1bcecf

## Log pattern

```text
canceling cache item key='%s' because keys request failed for '%s'
```

## Parameters

- Argument 1 — Output cache key: Text cache key reserved for the requested generated output.
- Argument 2 — Protected stream: Text application/stream name used to group pending DRM requests.

## Log examples

Illustrative examples; these are not captured server logs.

```text
canceling cache item key='/vod/movie.mp4/n_0_0_0.ts' because keys request failed for 'vod/movie.mp4'
```

## Short description

Pending media output canceled after DRM key failure.

## Trigger condition

A failed DRM-key notification finds the output cache item of a postponed VOD or DVR request.

## Explanation

The item is canceled and released, then its pending request is removed with the notified stream group.

## Possible causes

- A failed DRM-key notification finds the output cache item of a postponed VOD or DVR request.

## What to check

1. Inspect preceding key-server or DRM processing errors for the named stream; packaging cannot proceed without the required keys.

## Recommended action

Inspect preceding key-server or DRM processing errors for the named stream; packaging cannot proceed without the required keys.
