# Unhandled exception escaped a VOD request

Product: Nimble Streamer
Component: VOD transmuxing
Logging function: log_e
Log level: error
Documentation ID: log_e-de7831607ab9ffc2

## Log pattern

```text
unknown exception caught while processing request file='%s' chunk_name='%s' cache_item_key='%s'
```

## Parameters

- Argument 1 — Source media: Text filesystem path for local media, or source cache key for remote media.
- Argument 2 — Requested resource: Text name of the requested playlist, initialization segment, or media chunk.
- Argument 3 — Output cache key: Text cache key reserved for the requested generated output.

## Log examples

Illustrative examples; these are not captured server logs.

```text
unknown exception caught while processing request file='/media/movie.mp4' chunk_name='n_0_0_0.ts' cache_item_key='/vod/movie.mp4/n_0_0_0.ts'
```

## Short description

Unhandled exception escaped a VOD request.

## Trigger condition

An exception not derived from the standard exception type escapes the request handler into the worker loop.

## Explanation

The event is logged and the loop continues; this outer handler does not perform request-specific cache cleanup.

## Possible causes

- An exception not derived from the standard exception type escapes the request handler into the worker loop.

## What to check

1. Inspect earlier media, cache, and DRM diagnostics for this request, including whether output remains pending.

## Recommended action

Inspect earlier media, cache, and DRM diagnostics for this request, including whether output remains pending.
