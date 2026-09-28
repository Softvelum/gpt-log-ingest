# Transient input name lacks an application separator

Product: Nimble Streamer
Component: MPEG-TS stream registration and parsing
Logging function: log_e
Log level: error
Documentation ID: log_e-9cc9cee0b7de3a93

## Log pattern

```text
cannot add transient camera with given name [%s]. 'app/stream' parent should be used
```

## Parameters

- Argument 1 — Name: Rejected transient camera name.

## Log examples

Illustrative examples; these are not captured server logs.

```text
cannot add transient camera with given name [camera]. 'app/stream' parent should be used
```

## Short description

Transient input name lacks an application separator.

## Trigger condition

Splitting the requested transient camera name on slash produces fewer than two components.

## Explanation

Transient-stream creation is skipped, while normal camera registration and update tracking continue. The message uses the word parent, but the required shape is an application followed by a stream name.

## Possible causes

- Splitting the requested transient camera name on slash produces fewer than two components.

## What to check

1. Check the publisher-provided stream identifier.

## Recommended action

Supply an application/stream identifier.
