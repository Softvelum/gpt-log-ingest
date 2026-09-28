# Global connection count is already zero

Product: Nimble Streamer
Component: Streaming statistics
Logging function: log_e
Log level: error
Documentation ID: log_e-a45b2f0b4d5375ae

## Log pattern

```text
Statistics: disconnecting client ip='%s' host='%s' app='%s' stream='%s' but %u clients connected
```

## Parameters

- Argument 1 — Client address: Client IP address as text; an empty value can represent a request to create statistics records only.
- Argument 2 — Host: Text identifying the statistics host.
- Argument 3 — Application: Application name as text.
- Argument 4 — Stream: Stream name as text.
- Argument 5 — Global connections: Unsigned global client count, zero for this branch.

## Log examples

Illustrative examples; these are not captured server logs.

```text
Statistics: disconnecting client ip='192.0.2.10' host='media.example' app='live' stream='camera' but 0 clients connected
```

## Short description

Global connection count is already zero.

## Trigger condition

A valid stream disconnection has been processed, but the global connected-client count is zero.

## Explanation

The global counter is left at zero and the stream is still queued for statistics reporting.

## Possible causes

- Disconnection and global connection accounting do not match.

## What to check

1. Compare the reported stream disconnection with connection accounting.
