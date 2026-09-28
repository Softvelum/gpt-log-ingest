# DVR cleanup starts while playback is suspended

Product: Nimble Streamer
Component: DVR archive maintenance
Logging function: log_d
Log level: debug
Documentation ID: log_d-2d3e3f648ede33f3

## Log pattern

```text
delete dvr stream='%s/%s' [nl] target_depth=%llu older_than=%llu newer_than=%llu
```

## Parameters

- Argument 1 — Application: Text naming the application.
- Argument 2 — Stream: Text naming the stream.
- Argument 3 — Target depth: Unsigned retention depth in minutes; zero means no depth bound.
- Argument 4 — Older boundary: Unsigned UTC cutoff in seconds since the Unix epoch; zero means absent.
- Argument 5 — Newer boundary: Unsigned UTC lower bound in seconds since the Unix epoch; zero means absent.

## Log examples

Illustrative examples; these are not captured server logs.

```text
delete dvr stream='live/camera' [nl] target_depth=60 older_than=1700000060 newer_than=1700000000
```

## Short description

DVR cleanup starts while playback is suspended.

## Trigger condition

The alternate cleanup path marks a ready archive unavailable before running cleanup without holding the archive locks.

## Explanation

Playback requests see a nonready archive until cleanup finishes; load and update times then reset.

## Possible causes

- The alternate cleanup path marks a ready archive unavailable before running cleanup without holding the archive locks.

## What to check

1. Check the request boundaries and wait for the operation to finish.

## Recommended action

Check the request boundaries and wait for the operation to finish.
