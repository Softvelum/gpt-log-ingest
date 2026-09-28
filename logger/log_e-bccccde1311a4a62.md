# HTTP cache notification has no observer

Product: Nimble Streamer
Component: HTTP/2 and HTTP/3 request handling
Logging function: log_e
Log level: error
Documentation ID: log_e-bccccde1311a4a62

## Log pattern

```text
Http2Connection(%p) item (key='%s') is not observed
```

## Parameters

- Argument 1 — Connection identity: In-process connection object address, pointer displayed in hexadecimal; useful only for correlating this process lifetime.
- Argument 2 — Cache key: Requested cache-item key, text; identifies the origin resource.

## Log examples

Illustrative examples; these are not captured server logs.

```text
Http2Connection(0x12340000) item (key='https://origin.example/movie.mp4') is not observed
```

## Short description

HTTP cache notification has no observer.

## Trigger condition

A cache update arrives for a key absent from this connection’s observed-key map.

## Explanation

The notification is ignored; no pending response is advanced by this callback. The text names HTTP/2, but this shared handler also serves HTTP/3.

## Possible causes

- The observation has already been removed or notification delivery and connection bookkeeping are inconsistent.

## What to check

1. Check earlier request cancellation, response completion, and observer removal for the key.

## Recommended action

Investigate repeated unexpected notifications if requests stop progressing.
