# Windows file open failed

Product: Nimble Streamer
Component: Media file access
Logging function: log_e
Log level: error
Documentation ID: log_e-fe872b17b88dab6e

## Log pattern

```text
failed to open file='%s' error=%d
```

## Parameters

- Argument 1 — File: Requested runtime data-file path.
- Argument 2 — Error: Windows error code, printed in decimal.

One occurrence passes the unsigned Windows error type directly to a signed decimal conversion; the others store it in an int.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to open file='C:/media/segment.m4s' error=5
```

## Short description

Windows file open failed.

## Trigger condition

Windows rejects opening an existing media file or creating a file for memory mapping.

## Explanation

File mapping does not proceed. The create path requests a writable file; existing-file paths use their requested read/write access.

## Possible causes

- Windows rejects opening an existing media file or creating a file for memory mapping.

## What to check

1. Check the file path, permissions, sharing state, and Windows error.
