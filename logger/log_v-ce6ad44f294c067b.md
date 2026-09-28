# DVR archive settings are being updated

Product: Nimble Streamer
Component: DVR archive maintenance
Logging function: log_v
Log level: verbose
Documentation ID: log_v-ce6ad44f294c067b

## Log pattern

```text
updating dvr settings for stream='%s/%s' path='%s' depth=%u size=%llu format=%u/%llu/%llu
```

## Parameters

- Argument 1 — Application: Text naming the application.
- Argument 2 — Stream: Text naming the stream.
- Argument 3 — Archive directory: Text giving the recording directory.
- Argument 4 — Retention depth: Unsigned retention duration in minutes.
- Argument 5 — Storage limit: Unsigned maximum media storage bytes; zero means unlimited.
- Argument 6 — Storage version: Unsigned configured storage version: 1 separate segments, 2 shared data files.
- Argument 7 — Bucket limit: Unsigned maximum time-bucket size in MiB.
- Argument 8 — File limit: Unsigned maximum shared-file size in MiB.

## Log examples

Illustrative examples; these are not captured server logs.

```text
updating dvr settings for stream='live/camera' path='/data/dvr/live/camera' depth=60 size=104857600 format=2/1024/64
```

## Short description

DVR archive settings are being updated.

## Trigger condition

An existing archive receives a new settings set and storage path.

## Explanation

A path or UTC-offset change clears metadata and requires reload; otherwise settings are applied and disabled ID3 state is cleared.

## Possible causes

- An existing archive receives a new settings set and storage path.

## What to check

1. Check the new settings and any subsequent reload errors.

## Recommended action

Check the new settings and any subsequent reload errors.
