# Inactive session expired or was denied

Product: Nimble Streamer
Component: Playback session tracking
Logging function: log_v
Log level: verbose
Documentation ID: log_v-5e3f0ac7e02f285f

## Log pattern

```text
session %lu for %s#%s is expired (%s%s)
```

## Parameters

- Argument 1 — session: Numeric playback session identifier.
- Argument 2 — client: Client IP address.
- Argument 3 — protocol: Playback protocol suffix used in statistics.
- Argument 4 — host: Reported request host, with its port removed if that option is enabled.
- Argument 5 — URL: Requested resource URL.

## Log examples

Illustrative examples; these are not captured server logs.

```text
session 42 for 192.0.2.10#hls is expired (media.example/live/camera/playlist.m3u8)
```

## Short description

Inactive session expired or was denied.

## Trigger condition

An inactive session exceeds its configured retention time or is marked denied by the API.

## Explanation

The client-IP association, shared-session link when applicable, and session record are removed. The word expired also covers API denial.
