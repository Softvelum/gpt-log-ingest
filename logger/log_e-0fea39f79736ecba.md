# Advertising boundary cache storage allocation failed

Product: Nimble Streamer
Component: Advertising
Logging function: log_e
Log level: error
Documentation ID: log_e-0fea39f79736ecba

## Log pattern

```text
failed to allocate item, key='%s', size=%d
```

## Parameters

- Argument 1 — Cache key: Text identifying the affected media item in the cache.
- Argument 2 — Chunk bytes: Size of the completed remainder in bytes.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to allocate item, key='/app/stream/ad_1.ts', size=1048576
```

## Short description

Advertising boundary cache storage allocation failed.

## Trigger condition

Allocating cache data for the completed audio or MPEG-TS remainder fails.

## Explanation

The reserved cache item is canceled and released; the remainder request fails.

## Possible causes

- Allocating cache data for the completed audio or MPEG-TS remainder fails.

## What to check

1. Check cache capacity and memory for the requested remainder size.

## Recommended action

Check cache capacity and memory for the requested remainder size.
