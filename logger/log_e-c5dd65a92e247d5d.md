# Cannot open file for media access

Product: Nimble Streamer
Component: Media file access
Logging function: log_e
Log level: error
Documentation ID: log_e-c5dd65a92e247d5d

## Log pattern

```text
open_and_map_read_only_file open() failed file_name='%s' errno=%d
```

## Parameters

- Argument 1 — File: Requested runtime data-file path.
- Argument 2 — Error: Operating-system error number.

## Log examples

Illustrative examples; these are not captured server logs.

```text
open_and_map_read_only_file open() failed file_name='/data/media/segment.m4s' errno=2
```

## Short description

Cannot open file for media access.

## Trigger condition

Opening an existing file fails in buffered reading or memory mapping on a non-Windows platform.

## Explanation

No usable reader or mapping is returned. The generic mapping path can request writable access despite the read-only wording.

## Possible causes

- Opening an existing file fails in buffered reading or memory mapping on a non-Windows platform.

## What to check

1. Check the path, required access, and operating-system error.
