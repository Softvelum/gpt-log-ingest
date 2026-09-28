# SRT session registered for reporting

Product: Nimble Streamer
Component: SRT connections and session reporting
Logging function: log_d
Log level: debug
Documentation ID: log_d-ee67bc80dc79fe72

## Log pattern

```text
report SRT session stats for ip='%s' s='%s%s' sid=%u
```

## Parameters

- Argument 1 — Peer: Receiver IP address.
- Argument 2 — Host: Local address and port; IPv6 is enclosed in brackets.
- Argument 3 — Stream: Reporting application/stream path with a leading slash.
- Argument 4 — Session: Intended session identifier; the argument type differs from its conversion.

The session identifier is unsigned long, while the format uses an unsigned int conversion; their widths differ on common 64-bit platforms.

## Log examples

A portable sound example cannot be provided for the mismatched session-ID argument type.

## Short description

SRT session registered for reporting.

## Trigger condition

The session manager accepts a new SRT receiver session and returns a valid session ID.

## Explanation

The session can contribute client statistics and be checked for access denial. This log does not contain the traffic statistics themselves.

## Possible causes

- The session manager accepts a new SRT receiver session and returns a valid session ID.

## What to check

1. Match the receiver address and reporting stream with the intended session.
