# Advertising session response entry is invalid

Product: Nimble Streamer
Component: Advertising
Logging function: log_e
Log level: error
Documentation ID: log_e-7b770dbdd0bd4f00

## Log pattern

```text
invalid rules_response type=%d
```

## Parameters

- Argument 1 — JSON type: Integer JSON type code of the unexpected value.

## Log examples

Illustrative examples; these are not captured server logs.

```text
invalid rules_response type=6
```

## Short description

Advertising session response entry is invalid.

## Trigger condition

An element of the session rules response array is not a JSON object.

## Explanation

That element is skipped while remaining session responses are processed.

## Possible causes

- An element of the session rules response array is not a JSON object.

## What to check

1. Return an object for each session response entry.

## Recommended action

Return an object for each session response entry.
