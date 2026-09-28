# Accepted HTTP client dequeued

Product: Nimble Streamer
Component: HTTP request workers
Logging function: log_d
Log level: debug
Documentation ID: log_d-a810682bcc0dd2c5

## Log pattern

```text
adding client s=%d for processing (active=%d)
```

## Parameters

- Argument 1 — Socket: Integer socket descriptor.
- Argument 2 — Active entries: Local active-entry count including the dequeued candidate.

## Log examples

Illustrative examples; these are not captured server logs.

```text
adding client s=12 for processing (active=1)
```

## Short description

Accepted HTTP client dequeued.

## Trigger condition

An accepted connection was removed from the input queue while the worker had fewer than 5,000 active entries.

## Explanation

The local count is incremented, then duplicate-descriptor checking and session construction follow. This is an attempted assignment, not confirmation of successful session setup.
