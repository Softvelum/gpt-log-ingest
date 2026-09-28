# MKV loader failed during output generation

Product: Nimble Streamer
Component: VOD transmuxing
Logging function: log_e
Log level: error
Documentation ID: log_e-f14e5af9209cca65

## Log pattern

```text
BaseMKVLoaderException caught while processing file='%s' chunk_name='%s'
```

## Parameters

- Argument 1 — MKV filename: Text filesystem path of the source MKV file.
- Argument 2 — Requested resource: Text name of the requested playlist, initialization segment, or media chunk.

## Log examples

Illustrative examples; these are not captured server logs.

```text
BaseMKVLoaderException caught while processing file='/media/movie.mkv' chunk_name='n_0_0_0.ts'
```

## Short description

MKV loader failed during output generation.

## Trigger condition

A remaining MKV-loader exception reaches the local MKV transmuxing handler.

## Explanation

The requested output cache item is canceled with a transmuxing-failure reason and released.

## Possible causes

- A remaining MKV-loader exception reaches the local MKV transmuxing handler.

## What to check

1. Inspect the source MKV structure, selected tracks, and adjacent loader errors; this message provides no more specific loader cause.

## Recommended action

Inspect the source MKV structure, selected tracks, and adjacent loader errors; this message provides no more specific loader cause.
