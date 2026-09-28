# Invalid Speechmatics result item skipped

Product: Nimble Streamer
Component: Live speech transcription
Logging function: log_e
Log level: error
Documentation ID: log_e-1a122cd9bf50ca98

## Log pattern

```text
invalid result type=%d
```

## Parameters

- Argument 1 — Type: Numeric JSON type of the rejected result item.

## Log examples

Illustrative examples; these are not captured server logs.

```text
invalid result type=3
```

## Short description

Invalid Speechmatics result item skipped.

## Trigger condition

A translation results array item is not a JSON object.

## Explanation

The item is skipped. Valid remaining items may still supply translation text and timing; empty or reversed-time output is not queued.

## Possible causes

- A translation results array item is not a JSON object.

## What to check

1. Check the service translation results structure.
