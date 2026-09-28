# Origin playlist body parsing failed

Product: Nimble Streamer
Component: HTTP client sessions
Logging function: log_v
Log level: verbose
Documentation ID: log_v-087956fcbab4ed11

## Log pattern

```text
bad response body from the origin s=%d:
%.*s
```

## Parameters

- Argument 1 — Origin socket: Integer identifying the local origin socket.
- Argument 2 — Previously buffered bytes: Integer display limit before adding the failing read.
- Argument 3 — Buffered response: Raw previously accumulated origin response bytes.

## Log examples

Illustrative examples; these are not captured server logs.

```text
bad response body from the origin s=17:
invalid response
```

## Short description

Origin playlist body parsing failed.

## Trigger condition

New bytes fail HTTP parsing while a playlist body is being accumulated.

## Explanation

The client shutdown response is prepared. The printed buffer length excludes the just-read portion that caused parsing to fail.

## Possible causes

- New bytes fail HTTP parsing while a playlist body is being accumulated.

## What to check

1. Check origin HTTP framing and the declared body length; the shown prefix may not contain the failing bytes.

## Recommended action

Check origin HTTP framing and the declared body length; the shown prefix may not contain the failing bytes.
