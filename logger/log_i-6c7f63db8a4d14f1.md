# Saved state file could not be opened

Product: Nimble Streamer
Component: Playback session tracking
Logging function: log_i
Log level: info
Documentation ID: log_i-6c7f63db8a4d14f1

## Log pattern

```text
Failed to open file='%s'
```

## Parameters

- Argument 1 — file: Path of the saved session or recovery-statistics data file.

## Log examples

Illustrative examples; these are not captured server logs.

```text
Failed to open file='/var/lib/nimble/sessions.db'
```

## Short description

Saved state file could not be opened.

## Trigger condition

A session or recovery-statistics loader cannot open its selected file for reading.

## Explanation

The current loader returns without reading the file. Session loading can still emit a later completion summary after this failure.

## Possible causes

- The selected saved-state file could not be read.

## What to check

1. Check the named file, its parent directory, and nearby loading or saving messages.

## Recommended action

Restore file readability if this saved state is needed.
