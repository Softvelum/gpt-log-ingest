# Saved-session file is absent

Product: Nimble Streamer
Component: Playback session tracking
Logging function: log_i
Log level: info
Documentation ID: log_i-38816e1ff5bfb6da

## Log pattern

```text
Sessions file '%s' does not exist
```

## Parameters

- Argument 1 — file: Path of the saved session or recovery-statistics data file.

## Log examples

Illustrative examples; these are not captured server logs.

```text
Sessions file '/var/lib/nimble/sessions.db' does not exist
```

## Short description

Saved-session file is absent.

## Trigger condition

The selected session file is not found by the filesystem existence check.

## Explanation

Loading returns without restoring sessions. The message itself does not establish whether sessions were ever saved.
