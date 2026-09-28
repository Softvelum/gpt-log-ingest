# DASH packaging rejected the request

Product: Nimble Streamer
Component: VOD transmuxing
Logging function: log_e
Log level: error
Documentation ID: log_e-0f43b773ecfbf8ef

## Log pattern

```text
DashTransmuxingException caught while processing file='%s' chunk_name='%s' msg='%s'
```

## Parameters

- Argument 1 — Source media: Text filesystem path for local media, or source cache key for remote media.
- Argument 2 — Requested resource: Text name of the requested playlist, initialization segment, or media chunk.
- Argument 3 — Packaging detail: Text explanation carried by the DASH packaging exception.

## Log examples

Illustrative examples; these are not captured server logs.

```text
DashTransmuxingException caught while processing file='/media/movie.mp4' chunk_name='n_0_0_0.ts' msg='wrong segment index'
```

## Short description

DASH packaging rejected the request.

## Trigger condition

A DASH packaging exception reports an invalid track/segment selection, insufficient output space, or another specific packaging error.

## Explanation

The requested output cache item is canceled with a transmuxing-failure reason and released.

## Possible causes

- A DASH packaging exception reports an invalid track/segment selection, insufficient output space, or another specific packaging error.

## What to check

1. Use the detail to check the requested segment or track against the manifest, or output-buffer capacity when the detail names a size problem.

## Recommended action

Use the detail to check the requested segment or track against the manifest, or output-buffer capacity when the detail names a size problem.
