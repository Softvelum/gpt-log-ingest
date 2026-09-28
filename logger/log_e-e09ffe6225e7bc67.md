# Stream-check variant playlist URL is malformed

Product: Nimble Streamer
Component: Linux stream availability checks
Logging function: log_e
Log level: error
Documentation ID: log_e-e09ffe6225e7bc67

## Log pattern

```text
chunklist url='%s' is invalid for task='%s'
```

## Parameters

- Argument 1 — URL: Invalid variant playlist URL.
- Argument 2 — task: Panel-supplied stream-check task identifier.

## Log examples

Illustrative examples; these are not captured server logs.

```text
chunklist url='http://' is invalid for task='check-a'
```

## Short description

Stream-check variant playlist URL is malformed.

## Trigger condition

An HLS variant line begins with an HTTP scheme but lacks enough URL parts.

## Explanation

The checker records an invalid-URL result and ends body processing for the task.

## Possible causes

- The HLS playlist contains an incomplete absolute variant URL.

## What to check

1. Inspect the selected variant line in the master playlist.

## Recommended action

Correct the variant playlist URL.
