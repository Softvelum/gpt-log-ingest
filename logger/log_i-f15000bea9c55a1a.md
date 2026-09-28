# Saved-session loading returned

Product: Nimble Streamer
Component: Playback session tracking
Logging function: log_i
Log level: info
Documentation ID: log_i-f15000bea9c55a1a

## Log pattern

```text
Finished loading sessions, %u sessions, the next session id is %d
```

## Parameters

- Argument 1 — count: Combined active and inactive session count.
- Argument 2 — next identifier: Current highest session identifier plus one.

The next identifier is unsigned long but is passed to a signed-int conversion.

## Log examples

No portable example is provided for the mismatched next-identifier conversion.

## Short description

Saved-session loading returned.

## Trigger condition

The chosen loader returns and restored active sessions are reported as connected to statistics.

## Explanation

The count reflects sessions currently stored after the attempt, including partial progress. It does not prove the loader had no earlier open or parse error.
