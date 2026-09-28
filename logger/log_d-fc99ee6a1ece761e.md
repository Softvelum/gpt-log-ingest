# Session begins waiting for advertising rules

Product: Nimble Streamer
Component: HTTP request workers
Logging function: log_d
Log level: debug
Documentation ID: log_d-fc99ee6a1ece761e

## Log pattern

```text
[WrkT::AddAdsRulRO] session=%p
```

## Parameters

- Argument 1 — Session: Connection-session address used for diagnostic correlation.

## Log examples

Illustrative examples; these are not captured server logs.

```text
[WrkT::AddAdsRulRO] session=0x1000
```

## Short description

Session begins waiting for advertising rules.

## Trigger condition

A session requests rules-response notifications.

## Explanation

It joins the waiting set for its logical session identifier, creating an upstream subscription when necessary.
