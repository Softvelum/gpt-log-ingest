# FFmpeg diagnostic forwarded

Product: Nimble Streamer
Component: FFmpeg logging and media input
Logging function: log_always
Log level: info (always)
Documentation ID: log_always-5298ec97da576761

## Log pattern

```text
FFMPEG %s: %s
```

## Parameters

- Argument 1 — Severity: FFmpeg severity label: DEBUG, TRACE, VERBOSE, INFO, WARNING, ERROR, FATAL, or UNKNOWN.
- Argument 2 — Diagnostic: Formatted FFmpeg message text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
FFMPEG WARNING: illustrative library diagnostic
```

## Short description

FFmpeg diagnostic forwarded.

## Trigger condition

FFmpeg supplies a non-panic message at or below its configured FFmpeg log threshold and formatting produces nonempty text.

## Explanation

The formatted library message is forwarded without the normal Nimble level filter. A final newline is removed when present. The informational catalog tag does not replace the severity shown inside the message.

## Possible causes

- An FFmpeg library operation emitted a diagnostic at the included severity.

## What to check

1. Read the embedded severity and diagnostic text; the outer logging function alone does not indicate an error.

## Recommended action

Act on the specific FFmpeg diagnostic if it reports a failure.

## Related log messages

- [FFmpeg panic diagnostic](log_e-c95a2ae84bb441cd.md)
