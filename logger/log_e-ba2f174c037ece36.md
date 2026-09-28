# Cannot inspect opened mapping file

Product: Nimble Streamer
Component: Media file access
Logging function: log_e
Log level: error
Documentation ID: log_e-ba2f174c037ece36

## Log pattern

```text
open_and_map_read_only_file fstat() failed file_name='%s' fd=%d errno=%d
```

## Parameters

- Argument 1 — File: Requested runtime data-file path.
- Argument 2 — Descriptor: Open file descriptor.
- Argument 3 — Error: Operating-system error number.

## Log examples

Illustrative examples; these are not captured server logs.

```text
open_and_map_read_only_file fstat() failed file_name='/data/media/segment.m4s' fd=42 errno=5
```

## Short description

Cannot inspect opened mapping file.

## Trigger condition

The non-Windows file-status operation fails in the whole-file mapping path.

## Explanation

The descriptor is closed and marked invalid; no mapping is produced.

## Possible causes

- The non-Windows file-status operation fails in the whole-file mapping path.

## What to check

1. Check the file descriptor and operating-system error.
