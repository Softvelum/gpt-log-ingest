# MP4 loading failed during output generation

Product: Nimble Streamer
Component: VOD transmuxing
Logging function: log_e
Log level: error
Documentation ID: log_e-40878ced728deefd

## Log pattern

```text
BaseMP4LoaderException caught while processing file='%s' chunk='%s'
```

## Parameters

- Argument 1 — Source media: Text filesystem path for local media, or source cache key for remote media.
- Argument 2 — Requested resource: Text name of the requested playlist, initialization segment, or media chunk.

## Log examples

Illustrative examples; these are not captured server logs.

```text
BaseMP4LoaderException caught while processing file='/media/movie.mp4' chunk='n_0_0_0.ts'
```

## Short description

MP4 loading failed during output generation.

## Trigger condition

A base MP4-loader exception escapes media parsing or packaging inside the output worker.

## Explanation

The requested output cache item is canceled with a transmuxing-failure reason and released.

## Possible causes

- A base MP4-loader exception escapes media parsing or packaging inside the output worker.

## What to check

1. Check MP4 structure, track selections, available remote fragments, and preceding parser diagnostics.

## Recommended action

Check MP4 structure, track selections, available remote fragments, and preceding parser diagnostics.
