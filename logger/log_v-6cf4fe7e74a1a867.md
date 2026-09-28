# DVR archive loading started

Product: Nimble Streamer
Component: DVR archive storage
Logging function: log_v
Log level: verbose
Documentation ID: log_v-6cf4fe7e74a1a867

## Log pattern

```text
DvrArchive::LoadArchive app='%s' stream='%s' path='%s'
```

## Parameters

- Argument 1 — Application: Text naming the application.
- Argument 2 — Stream: Text naming the stream.
- Argument 3 — Archive directory: Text giving the archive or time-bucket directory.

## Log examples

Illustrative examples; these are not captured server logs.

```text
DvrArchive::LoadArchive app='live' stream='camera' path='/data/dvr/live/camera/1700000000000'
```

## Short description

DVR archive loading started.

## Trigger condition

Archive loading marks the archive as loading and begins scanning numeric time-bucket directories.

## Explanation

Recorded period and segment metadata is reconstructed.

## Possible causes

- Archive loading marks the archive as loading and begins scanning numeric time-bucket directories.
