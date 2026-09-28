# Signal stack trace could not be obtained

Product: Nimble Streamer
Component: Process signals and exception diagnostics
Logging function: log_d
Log level: debug
Documentation ID: log_d-4aac1de21c05c4cf

## Log pattern

```text
failed to get a backtrace
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to get a backtrace
```

## Short description

Signal stack trace could not be obtained.

## Trigger condition

The non-Windows default signal branch obtains stack addresses but cannot obtain their printable descriptions.

## Explanation

The handler returns without logging the stack trace.

## Possible causes

- The stack-description allocation returned no data.

## What to check

1. Check the preceding signal message.

## Recommended action

Investigate the originating unexpected signal.

## Related log messages

- [Signal stack trace recorded](log_e-e9fc0a473677506b.md)
- [Process signal received](log_i-f657a459e6235262.md)
