# Disabled traffic-counter diagnostic

Product: Nimble Streamer
Component: Streaming statistics
Logging function: log_d
Log level: debug
Documentation ID: log_d-f369cfad4d681eae

## Log pattern

```text
UpdateSessionStatistics updated statistics for %s:%s:%s in %d (%d), out %d (%d)
```

## Parameters

- Argument 1 — Client address: Client IP address as text; an empty value can represent a request to create statistics records only.
- Argument 2 — Host: Text identifying the statistics host.
- Argument 3 — URL: A dormant URL argument whose value is not established in the enclosing function.
- Argument 4 — Received increment: Requested received-byte increment, stored as an unsigned 64-bit value.
- Argument 5 — Received total: Stream received-byte total after the update, stored as an unsigned 64-bit value.
- Argument 6 — Sent increment: Requested sent-byte increment, stored as an unsigned 64-bit value.
- Argument 7 — Sent total: Stream sent-byte total after the update, stored as an unsigned 64-bit value.

The four byte values are 64-bit, while their conversions expect int. The disabled URL argument also lacks a current value.

## Log examples

This disabled, stale diagnostic cannot provide a sound emitted example.

## Short description

Disabled traffic-counter diagnostic.

## Trigger condition

The written diagnostic follows a successful traffic-counter update but is inside a disabled build branch.

## Explanation

It cannot be emitted by the current compiled branch. The dormant call refers to a URL value not supplied by the surrounding function and uses integer conversions for 64-bit byte counters.
