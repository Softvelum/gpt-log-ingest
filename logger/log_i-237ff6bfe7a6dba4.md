# Remote log reporting disabled without a server identifier

Product: Nimble Streamer
Component: Remote log reporting
Logging function: log_i
Log level: info
Documentation ID: log_i-237ff6bfe7a6dba4

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

Remote log reporting disabled without a server identifier.

## Trigger condition

The log reporting thread finds that the panel server identifier is empty.

## Explanation

The thread returns before entering the reporting loop. Its registration configuration is not reread during operation.

## Possible causes

- The panel server identifier is empty.

## What to check

1. Check the server registration identifier.

## Recommended action

Supply the required server identifier before startup.

## Related log messages

- [Remote log reporting starts](log_d-6b25cf40d455f58d.md)
- [Panel identity is empty](log_i-39894f35d89d1cd4.md)
