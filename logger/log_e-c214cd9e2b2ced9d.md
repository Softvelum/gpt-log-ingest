# Origin response header buffer is full

Product: Nimble Streamer
Component: HTTP client sessions
Logging function: log_e
Log level: error
Documentation ID: log_e-c214cd9e2b2ced9d

## Log pattern

```text
origin header buffer is too small resp size=%d, buf size=%d
```

## Parameters

- Argument 1 — Buffered bytes: Integer number of origin-response bytes already buffered.
- Argument 2 — Buffer capacity: Integer response-buffer capacity in bytes.

## Log examples

Illustrative examples; these are not captured server logs.

```text
origin header buffer is too small resp size=16384, buf size=16384
```

## Short description

Origin response header buffer is full.

## Trigger condition

The HTTP client or advertising downloader tries to read headers or buffered playlist data after filling the response buffer.

## Explanation

The client path prepares HTTP 500; the advertising downloader stops the failed session.

## Possible causes

- The HTTP client or advertising downloader tries to read headers or buffered playlist data after filling the response buffer.

## What to check

1. Check oversized origin headers and playlist length, and whether the response is valid HTTP.

## Recommended action

Check oversized origin headers and playlist length, and whether the response is valid HTTP.
