# Windows file view mapping failed

Product: Nimble Streamer
Component: Media file access
Logging function: log_e
Log level: error
Documentation ID: log_e-ef029b4026ace7c0

## Log pattern

```text
MapViewOfFile failed error=%d
```

## Parameters

- Argument 1 — Error: Windows error code, printed in decimal.

One occurrence supplies the unsigned Windows error type directly; other occurrences store the code in an int.

## Log examples

Illustrative examples; these are not captured server logs.

```text
MapViewOfFile failed error=8
```

## Short description

Windows file view mapping failed.

## Trigger condition

Windows cannot map the requested file view into memory.

## Explanation

No usable view is returned. This covers entire-file mappings and requested subregions.

## Possible causes

- Windows cannot map the requested file view into memory.

## What to check

1. Check the Windows error, file region, and available address space.
