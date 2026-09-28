# Icecast advertisement start pattern is invalid

Product: Nimble Streamer
Component: Advertising
Logging function: log_e
Log level: error
Documentation ID: log_e-c0a4ea7a500c704c

## Log pattern

```text
failed to parse out_regex='%s'
```

## Parameters

- Argument 1 — Start pattern: Text containing the metadata regular expression.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to parse out_regex='['
```

## Short description

Icecast advertisement start pattern is invalid.

## Trigger condition

Compiling the configured metadata pattern for a splice-out signal produces no valid expression.

## Explanation

The rule continues parsing; only a valid injection rule can subsequently be registered.

## Possible causes

- Compiling the configured metadata pattern for a splice-out signal produces no valid expression.

## What to check

1. Correct the metadata pattern used to detect advertisement starts.

## Recommended action

Correct the metadata pattern used to detect advertisement starts.
