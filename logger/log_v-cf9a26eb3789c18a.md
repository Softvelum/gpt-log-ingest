# Origin response without a body received

Product: Nimble Streamer
Component: HTTP client sessions
Logging function: log_v
Log level: verbose
Documentation ID: log_v-cf9a26eb3789c18a

## Log pattern

```text
origin s=%d response:
%.*s
```

## Parameters

- Argument 1 — Origin socket: Integer identifying the local origin socket.
- Argument 2 — Response byte limit: Integer byte count used as the text precision.
- Argument 3 — Response text: Raw complete bodyless origin response.

## Log examples

Illustrative examples; these are not captured server logs.

```text
origin s=17 response:
HTTP/1.0 204 OK


```

## Short description

Origin response without a body received.

## Trigger condition

Origin HTTP parsing finishes without a body pointer.

## Explanation

The entire buffered response is logged before status handling.

## Possible causes

- Origin HTTP parsing finishes without a body pointer.

## What to check

1. Not applicable; this is normal diagnostic progress. Use the socket identifier to follow the same request if troubleshooting.

## Recommended action

Not applicable; this is normal diagnostic progress. Use the socket identifier to follow the same request if troubleshooting.
