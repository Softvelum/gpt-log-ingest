# Cached playlist type is unrecognized

Product: Nimble Streamer
Component: HTTP client sessions
Logging function: log_e
Log level: error
Documentation ID: log_e-7d04d851cdeeced2

## Log pattern

```text
failed to detect playlist type url='%s' content_type='%s'
```

## Parameters

- Argument 1 — Origin URL: Text giving the origin request path and query string.
- Argument 2 — Content type: Text of the cached HTTP Content-Type value.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to detect playlist type url='/live/camera/playlist.m3u8' content_type='application/octet-stream'
```

## Short description

Cached playlist type is unrecognized.

## Trigger condition

A cached response URL and stored content type do not identify HLS, DASH, Smooth, or HDS.

## Explanation

The request is answered with HTTP 500 rather than processing that cached body as a playlist.

## Possible causes

- A cached response URL and stored content type do not identify HLS, DASH, Smooth, or HDS.

## What to check

1. Check that the origin route, playlist extension, and cached Content-Type describe a supported playlist.

## Recommended action

Check that the origin route, playlist extension, and cached Content-Type describe a supported playlist.
