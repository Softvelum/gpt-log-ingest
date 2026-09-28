# Origin HTTP response parsing failed

Product: Nimble Streamer
Component: HTTP client sessions
Logging function: log_v
Log level: verbose
Documentation ID: log_v-1dd98f0e9d84bc47

## Log pattern

```text
bad response from the origin s=%d:
%.*s
```

## Parameters

- Argument 1 — Origin socket: Integer identifying the local origin socket.
- Argument 2 — Response byte limit: Integer number of buffered response bytes to display.
- Argument 3 — Response text: Raw origin bytes interpreted as text, limited by the preceding length.

## Log examples

Illustrative examples; these are not captured server logs.

```text
bad response from the origin s=17:
invalid response
```

## Short description

Origin HTTP response parsing failed.

## Trigger condition

The buffered origin response cannot be parsed as valid HTTP.

## Explanation

The HTTP client prepares HTTP 500; cache and advertising downloads terminate.

## Possible causes

- The buffered origin response cannot be parsed as valid HTTP.

## What to check

1. Check the origin scheme and port and inspect the shown response for non-HTTP data or malformed headers.

## Recommended action

Check the origin scheme and port and inspect the shown response for non-HTTP data or malformed headers.
