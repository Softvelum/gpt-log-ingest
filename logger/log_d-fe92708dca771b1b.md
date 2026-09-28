# Origin media body parsing failed

Product: Nimble Streamer
Component: HTTP client sessions
Logging function: log_d
Log level: debug
Documentation ID: log_d-fe92708dca771b1b

## Log pattern

```text
bad response body from the origin s=%d
```

## Parameters

- Argument 1 — Origin socket: Integer identifying the local origin socket.

## Log examples

Illustrative examples; these are not captured server logs.

```text
bad response body from the origin s=17
```

## Short description

Origin media body parsing failed.

## Trigger condition

A new origin media-body block fails HTTP response parsing.

## Explanation

Origin downloading stops and the client shutdown response is prepared.

## Possible causes

- A new origin media-body block fails HTTP response parsing.

## What to check

1. Check origin body framing and whether the connection supplied a truncated or malformed response.

## Recommended action

Check origin body framing and whether the connection supplied a truncated or malformed response.
