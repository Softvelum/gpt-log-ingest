# Persisted recovery statistics loading finished

Product: Nimble Streamer
Component: Statistics recovery
Logging function: log_i
Log level: info
Documentation ID: log_i-5f56fe6d0a48f75a

## Log pattern

```text
Finished loading stats %s
```

## Parameters

- Argument 1 — Statistics file: Path of the accumulated or ready-to-report statistics file.

## Log examples

Illustrative examples; these are not captured server logs.

```text
Finished loading stats /var/cache/nimble/stats.json
```

## Short description

Persisted recovery statistics loading finished.

## Trigger condition

The statistics JSON has been parsed and its items processed.

## Explanation

Only records with a valid date and nonempty item map are retained. Completion does not assert that every item was accepted or that any statistics were loaded.

## Related log messages

- [Loading persisted recovery statistics](log_i-c695984a32e86fd0.md)
