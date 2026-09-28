# File-backed MP4 reading interrupted packaging

Product: Nimble Streamer
Component: VOD transmuxing
Logging function: log_e
Log level: error
Documentation ID: log_e-30baae4094ac110e

## Log pattern

```text
MP4FileReaderException caught while processing file='%s' chunk='%s' %s
```

## Parameters

- Argument 1 — Source media: Text filesystem path for local media, or source cache key for remote media.
- Argument 2 — Requested resource: Text name of the requested playlist, initialization segment, or media chunk.
- Argument 3 — Reader error: Text describing the failed read or seek with descriptor, byte offset, and operating-system error.

## Log examples

Illustrative examples; these are not captured server logs.

```text
MP4FileReaderException caught while processing file='/media/movie.mp4' chunk='n_0_0_0.ts' read fd=23 offset=0 size=131072 errno=5 'Input/output error'
```

## Short description

File-backed MP4 reading interrupted packaging.

## Trigger condition

The MP4 file reader raises a read or seek exception while the requested output is being processed.

## Explanation

The requested output cache item is canceled with a transmuxing-failure reason and released.

## Possible causes

- The MP4 file reader raises a read or seek exception while the requested output is being processed.

## What to check

1. Check the detailed I/O operation and storage error, and ensure the source file remains stable and readable.

## Recommended action

Check the detailed I/O operation and storage error, and ensure the source file remains stable and readable.
