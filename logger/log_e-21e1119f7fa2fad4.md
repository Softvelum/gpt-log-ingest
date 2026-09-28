# Cannot obtain Windows file size

Product: Nimble Streamer
Component: Media file access
Logging function: log_e
Log level: error
Documentation ID: log_e-21e1119f7fa2fad4

## Log pattern

```text
failed to get file='%s' size error=%d
```

## Parameters

- Argument 1 — File: Requested runtime data-file path.
- Argument 2 — Error: Windows error code, printed in decimal.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to get file='C:/media/segment.m4s' size error=5
```

## Short description

Cannot obtain Windows file size.

## Trigger condition

Reading the size of an opened Windows file fails before whole-file mapping.

## Explanation

The file handle is closed and mapping stops.

## Possible causes

- Reading the size of an opened Windows file fails before whole-file mapping.

## What to check

1. Inspect the Windows error and file accessibility.
