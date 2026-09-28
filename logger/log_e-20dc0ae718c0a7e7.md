# Playlist cache size is invalid

Product: Nimble Streamer
Component: Server configuration
Logging function: log_e
Log level: error
Documentation ID: log_e-20dc0ae718c0a7e7

## Log pattern

```text
invalid parameter value: %s. should be >= 0
```

## Parameters

- Argument 1 — Value: Original configuration value, text before numeric conversion; the applicable units depend on the named setting.

The value is logged as its original text. 

## Log examples

Illustrative examples; these are not captured server logs.

```text
invalid parameter value: 0. should be >= 0
```

## Short description

Playlist cache size is invalid.

## Trigger condition

While reading configuration, the transmuxer playlist-cache count is negative.

## Explanation

Configuration processing reports failure and the file loader stops at this line.

## Possible causes

- A configured value violates the setting-specific range or accepted choices.

## What to check

1. Check the setting on the rejected configuration line; it requires a nonnegative playlist count.

## Recommended action

Correct the value to a nonnegative playlist count and reload configuration.
