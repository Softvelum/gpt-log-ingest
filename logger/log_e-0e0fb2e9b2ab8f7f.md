# Advertising chunk cache reservation failed

Product: Nimble Streamer
Component: HTTP client sessions
Logging function: log_e
Log level: error
Documentation ID: log_e-0e0fb2e9b2ab8f7f

## Log pattern

```text
failed to reserve cache item, key=%s
```

## Parameters

- Argument 1 — Cache key: Text identifying the requested media or playlist cache item.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to reserve cache item, key=/live/camera/segment1.ts
```

## Short description

Advertising chunk cache reservation failed.

## Trigger condition

The cache returns no item for an advertising chunk reservation.

## Explanation

HTTP 500 is prepared before chunk generation can begin.

## Possible causes

- The cache returns no item for an advertising chunk reservation.

## What to check

1. Check cache allocation failures and advertising media preparation around this request.

## Recommended action

Check cache allocation failures and advertising media preparation around this request.
