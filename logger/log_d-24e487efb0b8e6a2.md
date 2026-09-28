# Origin pass-through request prepared

Product: Nimble Streamer
Component: HTTP client sessions
Logging function: log_d
Log level: debug
Documentation ID: log_d-24e487efb0b8e6a2

## Log pattern

```text
[CliS::POR] r='%s%s'
```

## Parameters

- Argument 1 — Origin host: Text containing the origin hostname and optional port.
- Argument 2 — Origin URL: Text giving the origin request path and query string.

## Log examples

Illustrative examples; these are not captured server logs.

```text
[CliS::POR] r='media.example:8080/live/camera/playlist.m3u8'
```

## Short description

Origin pass-through request prepared.

## Trigger condition

Caching could not reserve an item, or a request restarts in pass-through mode using its original Range header.

## Explanation

The translated origin request is recorded before connecting directly to fetch the response.

## Possible causes

- Caching could not reserve an item, or a request restarts in pass-through mode using its original Range header.

## What to check

1. Not applicable; this is normal diagnostic progress. Use the socket identifier to follow the same request if troubleshooting.

## Recommended action

Not applicable; this is normal diagnostic progress. Use the socket identifier to follow the same request if troubleshooting.
