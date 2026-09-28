# Advertising playlist subscription removal requested

Product: Nimble Streamer
Component: HTTP request workers
Logging function: log_d
Log level: debug
Documentation ID: log_d-779addab2d3661d0

## Log pattern

```text
[WrkT::RemAdsPlO] playlist_id='%s' session=%p
```

## Parameters

- Argument 1 — Playlist identifier: Text identifying the observed resource.
- Argument 2 — Session: Connection-session address used for diagnostic correlation.

## Log examples

Illustrative examples; these are not captured server logs.

```text
[WrkT::RemAdsPlO] playlist_id='example-item' session=0x1000
```

## Short description

Advertising playlist subscription removal requested.

## Trigger condition

A session requests removal from advertising-playlist notifications.

## Explanation

The playlist session set is looked up next; a missing set produces a separate error.
