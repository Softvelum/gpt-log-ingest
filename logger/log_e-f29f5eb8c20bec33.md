# Cannot allow an empty SRT poller

Product: Nimble Streamer
Component: SRT transport
Logging function: log_e
Log level: error
Documentation ID: log_e-f29f5eb8c20bec33

## Log pattern

```text
srt_epoll_set failed eid=%d flags=%d
```

## Parameters

- Argument 1 — Poller: SRT event-poller identifier.
- Argument 2 — Flags: Numeric setting mask for allowing an empty event poller.

## Log examples

Illustrative examples; these are not captured server logs.

```text
srt_epoll_set failed eid=1 flags=2
```

## Short description

Cannot allow an empty SRT poller.

## Trigger condition

The optional poller setting for allowing an empty socket set returns failure.

## Explanation

The failure is treated as noncritical and selector initialization continues. This setting is attempted only when the build and loaded library expose it.

## Possible causes

- The optional poller setting for allowing an empty socket set returns failure.

## What to check

1. Check library support for the empty-poller option if empty-poller errors recur.
