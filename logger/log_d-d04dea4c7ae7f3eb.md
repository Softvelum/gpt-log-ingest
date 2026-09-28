# Panel and server active-user counts differ

Product: Nimble Streamer
Component: WMSPanel synchronization
Logging function: log_d
Log level: debug
Documentation ID: log_d-d04dea4c7ae7f3eb

## Log pattern

```text
panel returns #%s as current active users but server thinks it is #%s
```

## Parameters

- Argument 1 — panel\_users: Panel count as a string.
- Argument 2 — server\_users: Current connected-user snapshot count as a decimal string.

## Log examples

Illustrative examples; these are not captured server logs.

```text
panel returns #9 as current active users but server thinks it is #10
```

## Short description

Panel and server active-user counts differ.

## Trigger condition

The panel Users value converted to a string differs from the string form of the current server snapshot count.

## Explanation

The next statistics report is marked as a full report to reconcile the counts. This comparison is textual, so differently represented values can also differ.
