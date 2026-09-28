# Regular stream checker lacks a panel identity

Product: Nimble Streamer
Component: Linux stream availability checks
Logging function: log_i
Log level: info
Documentation ID: log_i-5ace6d205885dd6d

## Log pattern

```text
streamCheckerThread stopped
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
streamCheckerThread stopped
```

## Short description

Regular stream checker lacks a panel identity.

## Trigger condition

The regular checker finds an empty configured panel server identifier.

## Explanation

The thread returns before event initialization or polling.
