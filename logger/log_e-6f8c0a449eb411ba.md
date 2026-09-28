# Trailing advertising boundary audio duration is unknown

Product: Nimble Streamer
Component: Advertising
Logging function: log_e
Log level: error
Documentation ID: log_e-6f8c0a449eb411ba

## Log pattern

```text
failed to get actual chunk duration, key=%s
```

## Parameters

- Argument 1 — Cache key: Text identifying the affected media item in the cache.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to get actual chunk duration, key=/app/stream/ad_1.ts
```

## Short description

Trailing advertising boundary audio duration is unknown.

## Trigger condition

Measuring the duration of audio remaining after the advertisement boundary fails.

## Explanation

The request returns without a successful remainder; its default status remains failed.

## Possible causes

- Measuring the duration of audio remaining after the advertisement boundary fails.

## What to check

1. Check the remaining source audio frames and codec.

## Recommended action

Check the remaining source audio frames and codec.
