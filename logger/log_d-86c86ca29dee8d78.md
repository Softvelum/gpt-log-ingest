# Advertising content subscription removal processed

Product: Nimble Streamer
Component: HTTP request workers
Logging function: log_d
Log level: debug
Documentation ID: log_d-86c86ca29dee8d78

## Log pattern

```text
[WrkT::RemAdsCoO] key='%s' p=%p
```

## Parameters

- Argument 1 — Content identifier: Text identifying the observed resource.
- Argument 2 — Session: Connection-session address used for diagnostic correlation.

## Log examples

Illustrative examples; these are not captured server logs.

```text
[WrkT::RemAdsCoO] key='example-item' p=0x1000
```

## Short description

Advertising content subscription removal processed.

## Trigger condition

The content waiting-session set exists and session removal has been attempted.

## Explanation

Remaining waiters keep their subscription; an empty set is removed from the worker and content manager.
