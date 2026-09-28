# Live-pull thread stopped

Product: Nimble Streamer
Component: Live input and publishing workers
Logging function: log_d
Log level: debug
Documentation ID: log_d-c2eb13e5dc42c339

## Log pattern

```text
LivePullThread stopped
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
LivePullThread stopped
```

## Short description

Live-pull thread stopped.

## Trigger condition

The thread observes an exit request and leaves its loop.

## Explanation

Receivers, probe records, settings, and receiver mappings have been cleared, and its delayed pull tasks have been released.
