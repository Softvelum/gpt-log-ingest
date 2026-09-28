# MPEG-TS output exceeds the available buffer

Product: Nimble Streamer
Component: VOD transmuxing
Logging function: log_e
Log level: error
Documentation ID: log_e-0f84ac66eaf82d61

## Log pattern

```text
buffer is too small to generate file='%s' chunk='%s' size=%u requested=%u
```

## Parameters

- Argument 1 — Source media: Text filesystem path for local media, or source cache key for remote media.
- Argument 2 — Requested resource: Text name of the requested playlist, initialization segment, or media chunk.
- Argument 3 — Buffer bytes: Unsigned 32-bit available output-buffer size in bytes.
- Argument 4 — Required bytes: Unsigned 32-bit required buffer size reported by the failed operation, in bytes.

## Log examples

Illustrative examples; these are not captured server logs.

```text
buffer is too small to generate file='/media/movie.mp4' chunk='n_0_0_0.ts' size=1048576 requested=1048764
```

## Short description

MPEG-TS output exceeds the available buffer.

## Trigger condition

Media packet generation raises a buffer-capacity exception while transmuxing a requested chunk.

## Explanation

The output item is canceled with the insufficient-memory reason and released.

## Possible causes

- Media packet generation raises a buffer-capacity exception while transmuxing a requested chunk.

## What to check

1. Compare the required size with the configured VOD transmuxing buffer; inspect unusually large chunks or media samples before increasing that capacity.

## Recommended action

Compare the required size with the configured VOD transmuxing buffer; inspect unusually large chunks or media samples before increasing that capacity.
