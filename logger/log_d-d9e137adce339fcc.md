# Remote log reporting stops

Product: Nimble Streamer
Component: Remote log reporting
Logging function: log_d
Log level: debug
Documentation ID: log_d-d9e137adce339fcc

## Log pattern

```text
LoggerThread stopped
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
LoggerThread stopped
```

## Short description

Remote log reporting stops.

## Trigger condition

The reporting loop detects an exit request or its interruptible wait ends for shutdown.

## Explanation

The reporting thread returns and sends no further batches.

## Related log messages

- [Remote log reporting starts](log_d-6b25cf40d455f58d.md)
