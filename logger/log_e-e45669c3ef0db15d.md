# Quick Sync encoder libraries are unavailable

Product: Nimble Streamer
Component: Transcoding output management
Logging function: log_e
Log level: error
Documentation ID: log_e-e45669c3ef0db15d

## Log pattern

```text
quick sync is not available
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
quick sync is not available
```

## Short description

Quick Sync encoder libraries are unavailable.

## Trigger condition

The native Quick Sync backend is requested in a supporting build, but its library-availability flag is false.

## Explanation

Encoder creation returns failure; there is no fallback in this selection branch.

## Possible causes

- The required optional encoder libraries did not load successfully.

## What to check

1. Check preceding Quick Sync library-loading diagnostics.

## Recommended action

Restore compatible libraries or select an available encoder backend.
