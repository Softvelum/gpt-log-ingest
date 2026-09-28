# Icecast chunk duration is invalid

Product: Nimble Streamer
Component: Server configuration
Logging function: log_e
Log level: error
Documentation ID: log_e-167deacf4325fad3

## Log pattern

```text
invalid parameter value: %s. should be >= 1 && <=10
```

## Parameters

- Argument 1 — Value: Original configuration value, text before numeric conversion; the applicable units depend on the named setting.

The value is logged as its original text. 

## Log examples

Illustrative examples; these are not captured server logs.

```text
invalid parameter value: 0. should be >= 1 && <=10
```

## Short description

Icecast chunk duration is invalid.

## Trigger condition

While reading configuration, the Icecast chunk duration is outside 1 through 10 seconds.

## Explanation

Configuration processing reports failure and the file loader stops at this line. The parsed value is assigned before the failure is returned; this message does not mean it was clamped to a valid range.

## Possible causes

- A configured value violates the setting-specific range or accepted choices.

## What to check

1. Check the setting on the rejected configuration line; it requires 1 through 10 seconds.

## Recommended action

Correct the value to 1 through 10 seconds and reload configuration.
