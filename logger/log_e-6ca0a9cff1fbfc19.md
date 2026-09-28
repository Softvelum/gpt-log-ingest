# Cannot seek to requested media-file offset

Product: Nimble Streamer
Component: Media file access
Logging function: log_e
Log level: error
Documentation ID: log_e-6ca0a9cff1fbfc19

## Log pattern

```text
open_and_map_read_only_file lseek() failed file_name='%s' errno=%d
```

## Parameters

- Argument 1 — File: Requested runtime data-file path.
- Argument 2 — Error: Operating-system error number.

## Log examples

Illustrative examples; these are not captured server logs.

```text
open_and_map_read_only_file lseek() failed file_name='/data/media/segment.m4s' errno=29
```

## Short description

Cannot seek to requested media-file offset.

## Trigger condition

Seeking to the requested starting offset fails before buffered reading on a non-Windows platform.

## Explanation

The reader is not returned and its file is cleaned up.

## Possible causes

- Seeking to the requested starting offset fails before buffered reading on a non-Windows platform.

## What to check

1. Check whether the input is seekable and the requested region is valid.
