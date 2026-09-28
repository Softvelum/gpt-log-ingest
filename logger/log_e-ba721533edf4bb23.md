# SRT retransmission bandwidth option unavailable

Product: Nimble Streamer
Component: SRT transport
Logging function: log_e
Log level: error
Documentation ID: log_e-ba721533edf4bb23

## Log pattern

```text
maxrexmitbw is not supported (requires SRT >= 1.5.3)
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
maxrexmitbw is not supported (requires SRT >= 1.5.3)
```

## Short description

SRT retransmission bandwidth option unavailable.

## Trigger condition

A maximum retransmission bandwidth setting is present but the loaded SRT version cannot be established as at least 1.5.3.

## Explanation

Only this option is skipped; applying subsequent socket settings continues.

## Possible causes

- A maximum retransmission bandwidth setting is present but the loaded SRT version cannot be established as at least 1.5.3.

## What to check

1. Check the loaded SRT version if this bandwidth limit is required.

## Recommended action

Use SRT 1.5.3 or later to apply this option.
