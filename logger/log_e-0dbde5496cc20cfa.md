# HTTP/3 cryptographic backlog setting is invalid

Product: Nimble Streamer
Component: Server configuration
Logging function: log_e
Log level: error
Documentation ID: log_e-0dbde5496cc20cfa

## Log pattern

```text
invalid parameter value: %s. should be > 0 && <= 255
```

## Parameters

- Argument 1 — Value: Original configuration value, text before numeric conversion; the applicable units depend on the named setting.

The value is logged as its original text. 

## Log examples

Illustrative examples; these are not captured server logs.

```text
invalid parameter value: 0. should be > 0 && <= 255
```

## Short description

HTTP/3 cryptographic backlog setting is invalid.

## Trigger condition

While reading configuration, the HTTP/3 cryptographic backlog limit is outside 1 through 255.

## Explanation

Configuration processing reports failure and the file loader stops at this line.

## Possible causes

- A configured value violates the setting-specific range or accepted choices.

## What to check

1. Check the setting on the rejected configuration line; it requires a backlog count from 1 through 255.

## Recommended action

Correct the value to a backlog count from 1 through 255 and reload configuration.
