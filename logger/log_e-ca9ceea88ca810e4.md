# MP4 structure read failed

Product: Nimble Streamer
Component: VOD transmuxing
Logging function: log_e
Log level: error
Documentation ID: log_e-ca9ceea88ca810e4

## Log pattern

```text
failed to load mp4 file structure for file='%s' %s
```

## Parameters

- Argument 1 — Media filename: Text filesystem path of the source media.
- Argument 2 — Reader error: Text describing the failing file operation, descriptor, offset, and operating-system error.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to load mp4 file structure for file='/media/movie.mp4' read fd=23 offset=0 size=131072 errno=5 'Input/output error'
```

## Short description

MP4 structure read failed.

## Trigger condition

The file-based MP4 reader raises a read or seek exception while loading a local source structure.

## Explanation

The file is unmapped and closed, the temporary media object is deleted, and source preparation returns failure.

## Possible causes

- The file-based MP4 reader raises a read or seek exception while loading a local source structure.

## What to check

1. Interpret the detailed file operation and error; verify the file descriptor remains valid and storage can read the source.

## Recommended action

Interpret the detailed file operation and error; verify the file descriptor remains valid and storage can read the source.
