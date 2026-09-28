# Advertising session request interval is invalid

Product: Nimble Streamer
Component: Server configuration
Logging function: log_e
Log level: error
Documentation ID: log_e-80c2669f7f7ce3dc

## Log pattern

```text
invalid parameter value: %s. should be >= 0 && <= 2592000
```

## Parameters

- Argument 1 — Value: Original configuration value, text before numeric conversion; the applicable units depend on the named setting.

The value is logged as its original text. 

## Log examples

Illustrative examples; these are not captured server logs.

```text
invalid parameter value: 0. should be >= 0 && <= 2592000
```

## Short description

Advertising session request interval is invalid.

## Trigger condition

While reading configuration, the advertising session-rules request interval is outside 0 through 2592000 after unsigned conversion.

## Explanation

Configuration processing reports failure and the file loader stops at this line.

## Possible causes

- A configured value violates the setting-specific range or accepted choices.

## What to check

1. Check the setting on the rejected configuration line; it requires an interval from 0 through 2592000.

## Recommended action

Correct the value to an interval from 0 through 2592000 and reload configuration.
