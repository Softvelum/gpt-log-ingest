# Immediate stream-check thread loop ended

Product: Nimble Streamer
Component: Linux stream availability checks
Logging function: log_d
Log level: debug
Documentation ID: log_d-07093da4b0d2cd1f

## Log pattern

```text
streamInstantlyCheckerThread stopped
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
streamInstantlyCheckerThread stopped
```

## Short description

Immediate stream-check thread loop ended.

## Trigger condition

The immediate checking loop ends after exit notification or a panel unknown-server response.

## Explanation

The checker event resources are then released and the thread returns.
