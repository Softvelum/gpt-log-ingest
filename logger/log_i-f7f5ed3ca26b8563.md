# Persisted recovery statistics are absent

Product: Nimble Streamer
Component: Statistics recovery
Logging function: log_i
Log level: info
Documentation ID: log_i-f7f5ed3ca26b8563

## Log pattern

```text
Recovery stats file '%s' does not exist
```

## Parameters

- Argument 1 — Statistics file: Path of the accumulated or ready-to-report statistics file.

## Log examples

Illustrative examples; these are not captured server logs.

```text
Recovery stats file '/var/cache/nimble/stats.json' does not exist
```

## Short description

Persisted recovery statistics are absent.

## Trigger condition

The filesystem existence check for a persisted statistics file returns false.

## Explanation

Loading that file stops before it is opened. The caller loads accumulated and ready-to-report statistics separately, so the other file may still be processed.

## Possible causes

- The file is absent or its existence could not be confirmed.

## What to check

1. Check which of the two statistics files is named.

## Related log messages

- [Loading persisted recovery statistics](log_i-c695984a32e86fd0.md)
