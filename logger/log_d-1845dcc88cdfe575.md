# Last advertising content subscription removed

Product: Nimble Streamer
Component: HTTP request workers
Logging function: log_d
Log level: debug
Documentation ID: log_d-1845dcc88cdfe575

## Log pattern

```text
[WrkT::RemAdsCoO] key='%s' p=%p empty set
```

## Parameters

- Argument 1 — Content identifier: Text identifying the observed resource.
- Argument 2 — Session: Connection-session address used for diagnostic correlation.

## Log examples

Illustrative examples; these are not captured server logs.

```text
[WrkT::RemAdsCoO] key='example-item' p=0x1000 empty set
```

## Short description

Last advertising content subscription removed.

## Trigger condition

No sessions remain in the content waiting set after removal.

## Explanation

The worker has unsubscribed from the content manager and erased the empty set.
