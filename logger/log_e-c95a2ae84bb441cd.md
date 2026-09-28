# FFmpeg panic diagnostic

Product: Nimble Streamer
Component: FFmpeg logging and media input
Logging function: log_e
Log level: error
Documentation ID: log_e-c95a2ae84bb441cd

## Log pattern

```text
FFMPEG PANIC: %s
```

## Parameters

- Argument 1 — Diagnostic: Formatted FFmpeg panic message text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
FFMPEG PANIC: illustrative panic diagnostic
```

## Short description

FFmpeg panic diagnostic.

## Trigger condition

FFmpeg sends a message at panic severity.

## Explanation

The message is formatted and forwarded through error logging without the callback threshold check. The callback itself does not terminate the process.

## Possible causes

- FFmpeg reported a panic-level condition.

## What to check

1. Inspect the included FFmpeg diagnostic and affected operation.

## Recommended action

Investigate the reported library failure.

## Related log messages

- [FFmpeg diagnostic forwarded](log_always-5298ec97da576761.md)
