# Advertising boundary cache reservation failed

Product: Nimble Streamer
Component: Advertising
Logging function: log_e
Log level: error
Documentation ID: log_e-9d7447fc79fdb4c5

## Log pattern

```text
failed to reserve item, key = '%s'
```

## Parameters

- Argument 1 — Cache key: Text identifying the affected media item in the cache.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to reserve item, key = '/app/stream/ad_1.ts'
```

## Short description

Advertising boundary cache reservation failed.

## Trigger condition

Reserving the new audio or MPEG-TS boundary chunk returns neither a new reservation nor an existing item.

## Explanation

The remainder request is marked failed.

## Possible causes

- Reserving the new audio or MPEG-TS boundary chunk returns neither a new reservation nor an existing item.

## What to check

1. Check cache availability for the derived boundary chunk.

## Recommended action

Check cache availability for the derived boundary chunk.
