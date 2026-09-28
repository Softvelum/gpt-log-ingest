# Shared origin session registered

Product: Nimble Streamer
Component: Playback session tracking
Logging function: log_v
Log level: verbose
Documentation ID: log_v-ecea6f4c77d1de2d

## Log pattern

```text
registered shared session id=%lu host='%s' shared_url='%s' url='%s'
```

## Parameters

- Argument 1 — session: Numeric playback session identifier.
- Argument 2 — host: Reported request host, with its port removed if that option is enabled.
- Argument 3 — shared URL: Common application-level tracking URL.
- Argument 4 — URL: Requested resource URL.

## Log examples

Illustrative examples; these are not captured server logs.

```text
registered shared session id=* host='media.example' shared_url='/live/http_origin_common' url='/live/camera/playlist.m3u8'
```

## Short description

Shared origin session registered.

## Trigger condition

A new shared application-level origin session is created.

## Explanation

The session uses a common stream name and has no client IP in its tracking record, allowing traffic reporting without counting it as a normal viewer connection.
