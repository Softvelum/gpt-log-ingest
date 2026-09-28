# Logging destination is unrecognized

Product: Nimble Streamer
Component: Server configuration
Logging function: log_e
Log level: error
Documentation ID: log_e-c0ae800b10f1e6ea

## Log pattern

```text
invalid config parameter name='%s' value='%s'
```

## Parameters

- Argument 1 — Setting: Configuration setting name, text.
- Argument 2 — Destinations: Entire original destination list, text.

The same format applies to access and session logging; it prints the whole value rather than only the invalid item.

## Log examples

Illustrative examples; these are not captured server logs.

```text
invalid config parameter name='ffmpeg_log_level' value='console'
```

## Short description

Logging destination is unrecognized.

## Trigger condition

An access-log or session-log destination in the comma-separated list is neither true, file, nor syslog.

## Explanation

Processing stops at the first unsupported item and configuration loading fails; earlier items may already have enabled a destination.

## Possible causes

- The destination list includes an unsupported value.

## What to check

1. Check the complete logged value and each comma-separated item.

## Recommended action

Use true, file, or syslog as destination items.
