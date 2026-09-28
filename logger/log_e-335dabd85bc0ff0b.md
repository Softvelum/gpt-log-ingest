# Advertising repackaging request is absent from the waiting map

Product: Nimble Streamer
Component: HTTP request workers
Logging function: log_e
Log level: error
Documentation ID: log_e-335dabd85bc0ff0b

## Log pattern

```text
[WrkT::RemAdsTmxRO] request_id='%d' session=%p not found
```

## Parameters

- Argument 1 — Request identifier: Unsigned 32-bit advertising repackaging request identifier displayed with a signed conversion.
- Argument 2 — Session: Connection-session address used for diagnostic correlation.

The request identifier is unsigned; values outside the signed range may appear negative.

## Log examples

Illustrative examples; these are not captured server logs.

```text
[WrkT::RemAdsTmxRO] request_id='42' session=0x1000 not found
```

## Short description

Advertising repackaging request is absent from the waiting map.

## Trigger condition

After clearing the upstream observer, removal finds no request entry.

## Explanation

No map entry is removed; the upstream request observer has already been cleared.

## Possible causes

- The request identifier has no waiting-map entry.

## What to check

1. Compare registration and removal messages for this request identifier.
