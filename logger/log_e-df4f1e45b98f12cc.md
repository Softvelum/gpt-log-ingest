# Saved state JSON could not be parsed

Product: Nimble Streamer
Component: Playback session tracking
Logging function: log_e
Log level: error
Documentation ID: log_e-df4f1e45b98f12cc

## Log pattern

```text
Failed to parse '%s'
```

## Parameters

- Argument 1 — file: Path of the saved session or recovery-statistics data file.

## Log examples

Illustrative examples; these are not captured server logs.

```text
Failed to parse '/var/lib/nimble/sessions.db'
```

## Short description

Saved state JSON could not be parsed.

## Trigger condition

The saved-session or recovery-statistics JSON parser returns no document.

## Explanation

The loader attempts to delete the invalid file and returns. In line-by-line session mode, earlier successfully restored records remain in memory.

## Possible causes

- The file, or a selected session-record line, is invalid JSON.

## What to check

1. Check the named file, its parent directory, and nearby loading or saving messages.

## Recommended action

Investigate damaged saved state; the rejected file is removed when deletion succeeds.
