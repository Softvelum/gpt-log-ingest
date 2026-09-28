# Client request translated to an origin request

Product: Nimble Streamer
Component: HTTP client sessions
Logging function: log_v
Log level: verbose
Documentation ID: log_v-ca9dc3444deaf069

## Log pattern

```text
redirecting '%s%s' to '%s%s'
```

## Parameters

- Argument 1 — Edge host: Text of the requested public hostname and optional port.
- Argument 2 — Edge URL: Text of the incoming request path and query.
- Argument 3 — Origin host: Text containing the origin hostname and optional port.
- Argument 4 — Origin URL: Text giving the origin request path and query string.

## Log examples

Illustrative examples; these are not captured server logs.

```text
redirecting 'edge.example/live/camera/playlist.m3u8' to 'media.example:8080/live/camera/playlist.m3u8'
```

## Short description

Client request translated to an origin request.

## Trigger condition

An HTTP client or cache downloader finishes preparing the upstream request.

## Explanation

The edge host/path and translated origin host/path are logged; this is an internal fetch mapping and does not itself send an HTTP redirect to the player.

## Possible causes

- An HTTP client or cache downloader finishes preparing the upstream request.

## What to check

1. Not applicable; this is normal diagnostic progress. Use the socket identifier to follow the same request if troubleshooting.

## Recommended action

Not applicable; this is normal diagnostic progress. Use the socket identifier to follow the same request if troubleshooting.
