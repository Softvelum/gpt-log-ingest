# Advertising content subscription removal requested

Product: Nimble Streamer
Component: HTTP request workers
Logging function: log_d
Log level: debug
Documentation ID: log_d-c8d1d88a9f2bb5e7

## Log pattern

```text
[WrkT::RemAdsCoO] content_id='%s' p=%p
```

## Parameters

- Argument 1 — Content identifier: Text identifying the observed resource.
- Argument 2 — Session: Connection-session address used for diagnostic correlation.

## Log examples

Illustrative examples; these are not captured server logs.

```text
[WrkT::RemAdsCoO] content_id='example-item' p=0x1000
```

## Short description

Advertising content subscription removal requested.

## Trigger condition

A session asks to stop observing advertising content.

## Explanation

The waiting-session set is looked up next; this message does not establish that it exists.
