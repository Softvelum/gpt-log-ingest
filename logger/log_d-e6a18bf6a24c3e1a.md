# Origin data requested for a new cache item

Product: Nimble Streamer
Component: HTTP client sessions
Logging function: log_d
Log level: debug
Documentation ID: log_d-e6a18bf6a24c3e1a

## Log pattern

```text
requesting data for cache item key='%s'
```

## Parameters

- Argument 1 — Cache key: Text identifying the requested media or playlist cache item.

## Log examples

Illustrative examples; these are not captured server logs.

```text
requesting data for cache item key='/live/camera/segment1.ts'
```

## Short description

Origin data requested for a new cache item.

## Trigger condition

The session has successfully reserved a new empty restreaming cache item.

## Explanation

Processing continues to connect to the origin and download its data.

## Possible causes

- The session has successfully reserved a new empty restreaming cache item.

## What to check

1. Not applicable; this is normal diagnostic progress. Use the socket identifier to follow the same request if troubleshooting.

## Recommended action

Not applicable; this is normal diagnostic progress. Use the socket identifier to follow the same request if troubleshooting.
