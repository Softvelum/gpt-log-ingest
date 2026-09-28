# Advertising chunk exceeds the working buffer

Product: Nimble Streamer
Component: Advertising
Logging function: log_e
Log level: error
Documentation ID: log_e-ef8dcb88b24a10e3

## Log pattern

```text
buffer is too small to generate advertizer chunk='%s'
```

## Parameters

- Argument 1 — Cache key: Text identifying the affected media item in the cache.

## Log examples

Illustrative examples; these are not captured server logs.

```text
buffer is too small to generate advertizer chunk='/app/stream/ad_1.ts'
```

## Short description

Advertising chunk exceeds the working buffer.

## Trigger condition

Generating an MPEG-TS or audio advertising chunk raises the buffer-too-small exception.

## Explanation

The chunk is not published; its pending cache item is canceled when the request is destroyed.

## Possible causes

- Generating an MPEG-TS or audio advertising chunk raises the buffer-too-small exception.

## What to check

1. Check the configured transmuxing buffer size against the advertising chunk size.

## Recommended action

Check the configured transmuxing buffer size against the advertising chunk size.
