# Stream connection count is already zero

Product: Nimble Streamer
Component: Streaming statistics
Logging function: log_e
Log level: error
Documentation ID: log_e-b8ee47e0e03cf501

## Log pattern

```text
StatisticsItem name_='%s' disconnecting client ip='%s' but connected=%u
```

## Parameters

- Argument 1 — Record name: Stream name.
- Argument 2 — Client address: Client IP address as text; an empty value can represent a request to create statistics records only.
- Argument 3 — Reported count: Unsigned stream connection count, zero in this branch.

## Log examples

Illustrative examples; these are not captured server logs.

```text
StatisticsItem name_='camera' disconnecting client ip='192.0.2.10' but connected=0
```

## Short description

Stream connection count is already zero.

## Trigger condition

A disconnection is being applied while the stream counter is zero.

## Explanation

The zero counter is not decremented. Other parent records continue to be processed.

## Possible causes

- A disconnect notification reached an already empty connection counter.

## What to check

1. Compare connect/disconnect events for the displayed record and address.
