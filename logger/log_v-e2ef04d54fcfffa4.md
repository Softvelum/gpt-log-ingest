# Managed task output channel closed

Product: Nimble Streamer
Component: Managed operating-system tasks
Logging function: log_v
Log level: verbose
Documentation ID: log_v-e2ef04d54fcfffa4

## Log pattern

```text
'%s' closed stdout fd=%d ev=0x%x
```

## Parameters

- Argument 1 — Command: Configured shell command text.
- Argument 2 — Output descriptor: Integer descriptor used by Nimble to read the task output.
- Argument 3 — Event mask: Hexadecimal socket-event bit mask.

## Log examples

Illustrative examples; these are not captured server logs.

```text
'echo ready' closed stdout fd=12 ev=0x10
```

## Short description

Managed task output channel closed.

## Trigger condition

A task event reports a hang-up or read-side closure. Available only on Linux builds.

## Explanation

The activity time is updated and closure is reported. This event handler does not itself reap the process or schedule a restart.
