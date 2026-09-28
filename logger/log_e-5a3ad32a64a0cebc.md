# Mapped media file close failed

Product: Nimble Streamer
Component: Media file access
Logging function: log_e
Log level: error
Documentation ID: log_e-5a3ad32a64a0cebc

## Log pattern

```text
munmap_and_close_file close failed. errno=%d
```

## Parameters

- Argument 1 — Error: Operating-system error number.

## Log examples

Illustrative examples; these are not captured server logs.

```text
munmap_and_close_file close failed. errno=9
```

## Short description

Mapped media file close failed.

## Trigger condition

Closing the file descriptor during memory-mapping cleanup fails on a non-Windows platform.

## Explanation

The close is not confirmed; mapping cleanup has already been attempted.

## Possible causes

- Closing the file descriptor during memory-mapping cleanup fails on a non-Windows platform.

## What to check

1. Inspect the error and preceding unmap result.
