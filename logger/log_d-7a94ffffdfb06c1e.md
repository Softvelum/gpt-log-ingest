# Remote VOD source mapping prepared

Product: Nimble Streamer
Component: HTTP client sessions
Logging function: log_d
Log level: debug
Documentation ID: log_d-7a94ffffdfb06c1e

## Log pattern

```text
origin_host='%s' origin_url='%s'
```

## Parameters

- Argument 1 — Origin host: Text containing the origin hostname and optional port.
- Argument 2 — Origin URL: Text giving the origin request path and query string.

## Log examples

Illustrative examples; these are not captured server logs.

```text
origin_host='media.example:8080' origin_url='/live/camera/playlist.m3u8'
```

## Short description

Remote VOD source mapping prepared.

## Trigger condition

A remote VOD request needs its source item after transmuxed-output lookup.

## Explanation

The normalized source host and URL are recorded before reserving or reusing the source cache item.

## Possible causes

- A remote VOD request needs its source item after transmuxed-output lookup.

## What to check

1. Not applicable; this is normal diagnostic progress. Use the socket identifier to follow the same request if troubleshooting.

## Recommended action

Not applicable; this is normal diagnostic progress. Use the socket identifier to follow the same request if troubleshooting.
