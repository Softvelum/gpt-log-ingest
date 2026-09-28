# DVR media file still has active users

Product: Nimble Streamer
Component: DVR archive storage
Logging function: log_d
Log level: debug
Documentation ID: log_d-f45bbd1e2e7be941

## Log pattern

```text
keeping segment f='%s' use_count=%ld
```

## Parameters

- Argument 1 — Storage file: Text naming the recorded media storage file.
- Argument 2 — Reference count: Signed long count of references holding the file, including the current local reference.

## Log examples

Illustrative examples; these are not captured server logs.

```text
keeping segment f='/data/dvr/live/camera/1700000000000.dat' use_count=2
```

## Short description

DVR media file still has active users.

## Trigger condition

All data segments have been detached but other references still hold the storage file.

## Explanation

Physical deletion is deferred until a later cleanup.

## Possible causes

- All data segments have been detached but other references still hold the storage file.

## What to check

1. Check active playback or current recording use if delayed deletion persists.

## Recommended action

Check active playback or current recording use if delayed deletion persists.
