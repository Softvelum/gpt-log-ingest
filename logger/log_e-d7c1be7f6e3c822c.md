# Log retention count is invalid

Product: Nimble Streamer
Component: Server configuration
Logging function: log_e
Log level: error
Documentation ID: log_e-d7c1be7f6e3c822c

## Log pattern

```text
invalid parameter value: %s (should be > 1)
```

## Parameters

- Argument 1 — Value: Original configuration value, text before numeric conversion; the applicable units depend on the named setting.

The value is logged as its original text. 

## Log examples

Illustrative examples; these are not captured server logs.

```text
invalid parameter value: 0 (should be > 1)
```

## Short description

Log retention count is invalid.

## Trigger condition

While reading configuration, the number of retained log files is below two.

## Explanation

Configuration processing reports failure and the file loader stops at this line.

## Possible causes

- A configured value violates the setting-specific range or accepted choices.

## What to check

1. Check the setting on the rejected configuration line; it requires at least two retained files.

## Recommended action

Correct the value to at least two retained files and reload configuration.
