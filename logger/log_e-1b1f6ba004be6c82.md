# Inactive HLS publisher removed

Product: Nimble Streamer
Component: HTTP media delivery
Logging function: log_e
Log level: error
Documentation ID: log_e-1b1f6ba004be6c82

## Log pattern

```text
inactive hls publisher removed: app=%s, stream=%s, url=%s

```

## Parameters

- Argument 1 — Application: Source application name.
- Argument 2 — Stream: Source stream name.
- Argument 3 — URL: Publishing destination URL.

## Log examples

Illustrative examples; these are not captured server logs.

```text
inactive hls publisher removed: app=live, stream=news, url=https://upload.example.net/live/

```

## Short description

Inactive HLS publisher removed.

## Trigger condition

A publisher reaches three inactivity checks without a completed request resetting its counter.

## Explanation

The publisher is removed and settings resynchronization is requested, allowing recreation. Checks occur when more than ten seconds have passed or wall time moves backward.

## Possible causes

- A publisher reaches three inactivity checks without a completed request resetting its counter.

## What to check

1. Check source chunk updates and whether destination requests complete.
