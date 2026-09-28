# New advertising chunk generation failed to start

Product: Nimble Streamer
Component: HTTP client sessions
Logging function: log_e
Log level: error
Documentation ID: log_e-ab72d0ffab707b3f

## Log pattern

```text
failed to process advertising chunk request key=%s
```

## Parameters

- Argument 1 — Cache key: Text identifying the requested media or playlist cache item.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to process advertising chunk request key=/live/camera/segment1.ts
```

## Short description

New advertising chunk generation failed to start.

## Trigger condition

A newly reserved advertising chunk has an observer, but submitting its generation request fails.

## Explanation

The request is rejected with HTTP 500; HTTP/2 or HTTP/3 also removes the observer.

## Possible causes

- A newly reserved advertising chunk has an observer, but submitting its generation request fails.

## What to check

1. Check adjacent advertising worker and transcoding errors associated with this cache key.

## Recommended action

Check adjacent advertising worker and transcoding errors associated with this cache key.
