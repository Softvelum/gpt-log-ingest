# HLS processing failed for MKV media

Product: Nimble Streamer
Component: VOD transmuxing
Logging function: log_e
Log level: error
Documentation ID: log_e-bf2fa597ad5c4070

## Log pattern

```text
HLSProcessorException caught while processing file='%s' chunk='%s'
```

## Parameters

- Argument 1 — MKV filename: Text filesystem path of the source MKV file.
- Argument 2 — Requested resource: Text name of the requested playlist, initialization segment, or media chunk.

## Log examples

Illustrative examples; these are not captured server logs.

```text
HLSProcessorException caught while processing file='/media/movie.mkv' chunk='n_0_0_0.ts'
```

## Short description

HLS processing failed for MKV media.

## Trigger condition

An HLS-processing exception escapes playlist or chunk generation for a local MKV source.

## Explanation

The requested output cache item is canceled with a transmuxing-failure reason and released.

## Possible causes

- An HLS-processing exception escapes playlist or chunk generation for a local MKV source.

## What to check

1. Check the named source media and preceding parser diagnostics; verify that the requested chunk and selected tracks belong to that media.

## Recommended action

Check the named source media and preceding parser diagnostics; verify that the requested chunk and selected tracks belong to that media.
