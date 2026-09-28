# Failed fragmented item lacks its first fragment

Product: Nimble Streamer
Component: Media data cache
Logging function: log_e
Log level: error
Documentation ID: log_e-c0694db26b4b8b0f

## Log pattern

```text
unable to remove fragmented item key='%s' fragments=%llu
```

## Parameters

- Argument 1 — key: Cache lookup key identifying the stored item.
- Argument 2 — fragment slots: Number of slots in the fragment array, including empty slots.

## Log examples

Illustrative examples; these are not captured server logs.

```text
unable to remove fragmented item key='movie' fragments=0
```

## Short description

Failed fragmented item lacks its first fragment.

## Trigger condition

A marked, unused fragmented item has no fragment slots or an empty first slot during replacement.

## Explanation

Cleanup cannot use its expected first fragment, so the reservation reports that the item still exists.

## Possible causes

- The cleanup path expected an allocated first fragment but none was present.

## What to check

1. Check allocation and removal history for fragment zero.

## Recommended action

Investigate the inconsistent fragmented item.
