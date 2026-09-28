# Advertising repackaging completion delivered

Product: Nimble Streamer
Component: HTTP request workers
Logging function: log_d
Log level: debug
Documentation ID: log_d-87759bbdae2a281e

## Log pattern

```text
[WrkT::AdTmxReqCd] req='%d' p=%p
```

## Parameters

- Argument 1 — Request identifier: Unsigned 32-bit advertising repackaging request identifier displayed with a signed conversion.
- Argument 2 — Session: Connection-session address used for diagnostic correlation.

The request identifier is unsigned; values outside the signed range may appear negative.

## Log examples

Illustrative examples; these are not captured server logs.

```text
[WrkT::AdTmxReqCd] req='42' p=0x1000
```

## Short description

Advertising repackaging completion delivered.

## Trigger condition

A queued request completion has a matching waiting session.

## Explanation

The completion callback is invoked and the session is deleted afterward if finished.
