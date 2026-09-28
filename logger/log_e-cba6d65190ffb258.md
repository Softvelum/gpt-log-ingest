# Cannot obtain media file size

Product: Nimble Streamer
Component: Media file access
Logging function: log_e
Log level: error
Documentation ID: log_e-cba6d65190ffb258

## Log pattern

```text
open_and_map_read_only_file fstat() failed file_name='%s' errno=%d
```

## Parameters

- Argument 1 — File: Requested runtime data-file path.
- Argument 2 — Error: Operating-system error number.

## Log examples

Illustrative examples; these are not captured server logs.

```text
open_and_map_read_only_file fstat() failed file_name='/data/media/segment.m4s' errno=5
```

## Short description

Cannot obtain media file size.

## Trigger condition

The file-status call fails after opening the file for a region mapping on a non-Windows platform.

## Explanation

The mapping reader is rejected before its requested range is checked.

## Possible causes

- The file-status call fails after opening the file for a region mapping on a non-Windows platform.

## What to check

1. Inspect the file-status error and file accessibility.
