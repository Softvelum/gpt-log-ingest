# Stream-check body handler received an unsupported protocol

Product: Nimble Streamer
Component: Linux stream availability checks
Logging function: log_e
Log level: error
Documentation ID: log_e-5a31af13bfbd834c

## Log pattern

```text
invalid ProcessReceivedBody() call task='%s'
```

## Parameters

- Argument 1 — task: Panel-supplied stream-check task identifier.

## Log examples

No ordinary trigger is established because supported RTMP input has a separate processing path.

## Short description

Stream-check body handler received an unsupported protocol.

## Trigger condition

Body interpretation is entered for a task that is neither HTTP nor RTSP.

## Explanation

The task status becomes unknown and body processing requests termination. Ordinary RTMP handling uses its separate response processor.

## Possible causes

- An unexpected protocol state reached the HTTP/RTSP body handler.

## What to check

1. Compare task protocol and prior state messages.

## Recommended action

Investigate the unexpected task state.
