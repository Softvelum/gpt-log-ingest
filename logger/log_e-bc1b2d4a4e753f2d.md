# Windows mapping handle close failed

Product: Nimble Streamer
Component: Media file access
Logging function: log_e
Log level: error
Documentation ID: log_e-bc1b2d4a4e753f2d

## Log pattern

```text
CloseHandle h_mapping=%d failed error=%d
```

## Parameters

- Argument 1 — Handle: Mapping-object handle represented as an integer.
- Argument 2 — Error: Windows error code, printed in decimal.

## Log examples

Illustrative examples; these are not captured server logs.

```text
CloseHandle h_mapping=100 failed error=6
```

## Short description

Windows mapping handle close failed.

## Trigger condition

Closing the mapping-object handle fails during file cleanup.

## Explanation

Cleanup continues to the file handle.

## Possible causes

- Closing the mapping-object handle fails during file cleanup.

## What to check

1. Inspect the Windows error and prior mapping cleanup.
