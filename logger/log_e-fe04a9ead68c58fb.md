# RTMP chunk size setting is unsupported

Product: Nimble Streamer
Component: Server configuration
Logging function: log_e
Log level: error
Documentation ID: log_e-fe04a9ead68c58fb

## Log pattern

```text
invalid parameter value: %s. should be 1M / 2M / 3M
```

## Parameters

- Argument 1 — Value: Original configuration value, text before numeric conversion; the applicable units depend on the named setting.

The value is logged as its original text. 

## Log examples

Illustrative examples; these are not captured server logs.

```text
invalid parameter value: 0. should be 1M / 2M / 3M
```

## Short description

RTMP chunk size setting is unsupported.

## Trigger condition

While reading configuration, the RTMP chunk-size text is not exactly 1M, 2M, or 3M.

## Explanation

Configuration processing reports failure and the file loader stops at this line.

## Possible causes

- A configured value violates the setting-specific range or accepted choices.

## What to check

1. Check the setting on the rejected configuration line; it requires exactly 1M, 2M, or 3M.

## Recommended action

Correct the value to exactly 1M, 2M, or 3M and reload configuration.
