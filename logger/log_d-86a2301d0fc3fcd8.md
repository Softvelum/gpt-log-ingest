# Regular stream-check thread loop ended

Product: Nimble Streamer
Component: Linux stream availability checks
Logging function: log_d
Log level: debug
Documentation ID: log_d-86a2301d0fc3fcd8

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

Regular stream-check thread loop ended.

## Trigger condition

The regular checking loop ends after exit notification or a panel unknown-server response.

## Explanation

The checker event resources are then released and the thread returns.
