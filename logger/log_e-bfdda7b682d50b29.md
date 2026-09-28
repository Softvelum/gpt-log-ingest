# HTTP/3 socket buffer setting is invalid

Product: Nimble Streamer
Component: Server configuration
Logging function: log_e
Log level: error
Documentation ID: log_e-bfdda7b682d50b29

## Log pattern

```text
invalid parameter value: %s. should be >= -1
```

## Parameters

- Argument 1 — Value: Original configuration value, text before numeric conversion; the applicable units depend on the named setting.

The value is logged as its original text. Several settings share this pattern; identify the setting from the rejected configuration line.

## Log examples

Illustrative examples; these are not captured server logs.

```text
invalid parameter value: 0. should be >= -1
```

## Short description

HTTP/3 socket buffer setting is invalid.

## Trigger condition

While reading configuration, an HTTP/3 UDP receive or send buffer value converts to less than -1.

## Explanation

Configuration processing reports failure and the file loader stops at this line.

## Possible causes

- A configured value violates the setting-specific range or accepted choices.

## What to check

1. Check the setting on the rejected configuration line; it requires a value of -1 or a nonnegative buffer size.

## Recommended action

Correct the value to a value of -1 or a nonnegative buffer size and reload configuration.
