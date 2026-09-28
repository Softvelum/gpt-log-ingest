# Buffered media file close failed

Product: Nimble Streamer
Component: Media file access
Logging function: log_e
Log level: error
Documentation ID: log_e-f46321ff2c5bbf47

## Log pattern

```text
failed to close file errno=%d
```

## Parameters

- Argument 1 — Error: Operating-system error number.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to close file errno=5
```

## Short description

Buffered media file close failed.

## Trigger condition

Closing a buffered-reader file descriptor returns an error on a non-Windows platform.

## Explanation

The reader clears its stored descriptor regardless. This can happen during cleanup or after a successful read, so it does not by itself imply that buffered data is unavailable.

## Possible causes

- Closing a buffered-reader file descriptor returns an error on a non-Windows platform.

## What to check

1. Inspect the operating-system error and preceding read result.
