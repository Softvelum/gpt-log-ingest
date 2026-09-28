# Whisper library diagnostic

Product: Nimble Streamer
Component: Whisper transcription
Logging function: log_always
Log level: info (always)
Documentation ID: log_always-777846ecbcfc5d7c

## Log pattern

```text
WHISPER %s: %s
```

## Parameters

- Argument 1 — Severity: Library severity label NONE, INFO, WARN, ERROR, DEBUG, or UNKNOWN.
- Argument 2 — Message: Diagnostic text supplied by the Whisper library.

## Log examples

Illustrative examples; these are not captured server logs.

```text
WHISPER INFO: model loaded
```

## Short description

Whisper library diagnostic.

## Trigger condition

A Whisper library diagnostic passes the configured Whisper-specific severity filter.

## Explanation

The library text is forwarded through always-logged output with an informational tag. Its embedded label retains the library severity, so the outer tag alone does not identify an error.

## Possible causes

- A Whisper library diagnostic passes the configured Whisper-specific severity filter.

## What to check

1. Read the embedded severity and library diagnostic text.
