# Saved state file cannot be opened for writing

Product: Nimble Streamer
Component: Playback session tracking
Logging function: log_e
Log level: error
Documentation ID: log_e-b12d4ee1279fac84

## Log pattern

```text
failed to open '%s' file to save sessions
```

## Parameters

- Argument 1 — file: Path of the saved session or recovery-statistics data file.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to open '/var/lib/nimble/sessions.db' file to save sessions
```

## Short description

Saved state file cannot be opened for writing.

## Trigger condition

Opening a session or recovery-statistics file for truncating write fails.

## Explanation

The save attempt returns before writing. The message says sessions even when the file holds recovery statistics.

## Possible causes

- The target state file cannot be opened for writing.

## What to check

1. Check the named file, its parent directory, and nearby loading or saving messages.

## Recommended action

Restore write access so the state can be saved.
