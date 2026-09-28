# Stream-check task URL is invalid

Product: Nimble Streamer
Component: Linux stream availability checks
Logging function: log_e
Log level: error
Documentation ID: log_e-b64abfaf9a1539cf

## Log pattern

```text
task is invalid url='%s'
```

## Parameters

- Argument 1 — URL: Task URL supplied by the panel.

## Log examples

Illustrative examples; these are not captured server logs.

```text
task is invalid url='invalid'
```

## Short description

Stream-check task URL is invalid.

## Trigger condition

Splitting a new task URL produces fewer than the required protocol and host parts.

## Explanation

The task remains invalid and is reported without opening an origin connection.

## Possible causes

- The task URL lacks the required protocol and host parts.

## What to check

1. Check the task URL submitted for monitoring.

## Recommended action

Correct the monitored URL.
