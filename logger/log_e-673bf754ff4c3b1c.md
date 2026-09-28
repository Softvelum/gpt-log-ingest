# libvpx encoder libraries are unavailable

Product: Nimble Streamer
Component: Transcoding output management
Logging function: log_e
Log level: error
Documentation ID: log_e-673bf754ff4c3b1c

## Log pattern

```text
libvpx is unavailable
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
libvpx is unavailable
```

## Short description

libvpx encoder libraries are unavailable.

## Trigger condition

The native libvpx backend is requested in a supporting build, but its library-availability flag is false.

## Explanation

Encoder creation returns failure; there is no fallback in this selection branch.

## Possible causes

- The required optional encoder libraries did not load successfully.

## What to check

1. Check preceding libvpx library-loading diagnostics.

## Recommended action

Restore compatible libraries or select an available encoder backend.
