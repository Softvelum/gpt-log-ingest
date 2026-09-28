# MPEG-TS generation failed

Product: Nimble Streamer
Component: VOD transmuxing
Logging function: log_e
Log level: error
Documentation ID: log_e-edd324f3900e25ea

## Log pattern

```text
BaseMPEG2TSException caught while processing file='%s' chunk='%s'
```

## Parameters

- Argument 1 — Source media: Text filesystem path for local media, or source cache key for remote media.
- Argument 2 — Requested resource: Text name of the requested playlist, initialization segment, or media chunk.

## Log examples

Illustrative examples; these are not captured server logs.

```text
BaseMPEG2TSException caught while processing file='/media/movie.mp4' chunk='n_0_0_0.ts'
```

## Short description

MPEG-TS generation failed.

## Trigger condition

A MPEG-TS processing exception other than the separately handled buffer-capacity exception reaches the MKV or general transmuxing handler.

## Explanation

The requested output cache item is canceled with a transmuxing-failure reason and released.

## Possible causes

- A MPEG-TS processing exception other than the separately handled buffer-capacity exception reaches the MKV or general transmuxing handler.

## What to check

1. Check the named source media and preceding parser diagnostics; verify that the requested chunk and selected tracks belong to that media.

## Recommended action

Check the named source media and preceding parser diagnostics; verify that the requested chunk and selected tracks belong to that media.
