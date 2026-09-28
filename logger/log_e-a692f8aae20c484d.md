# Accepted SRT publisher has invalid stream ID

Product: Nimble Streamer
Component: SRT publishing authentication
Logging function: log_e
Log level: error
Documentation ID: log_e-a692f8aae20c484d

## Log pattern

```text
invalid srt streamid=%s, s=%d
```

## Parameters

- Argument 1 — Stream ID: Requested SRT stream identifier.
- Argument 2 — Socket: SRT socket identifier.

## Log examples

Illustrative examples; these are not captured server logs.

```text
invalid srt streamid=news, s=101
```

## Short description

Accepted SRT publisher has invalid stream ID.

## Trigger condition

After acceptance, the stream ID cannot be split into application and stream components.

## Explanation

The socket is removed from the poller and closed before the camera is created.

## Possible causes

- After acceptance, the stream ID cannot be split into application and stream components.

## What to check

1. Check the publisher stream-ID structure.

## Recommended action

Supply a valid application/stream identifier.
