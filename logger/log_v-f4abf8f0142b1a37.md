# DVR archive registered for recording

Product: Nimble Streamer
Component: DVR
Logging function: log_v
Log level: verbose
Documentation ID: log_v-f4abf8f0142b1a37

## Log pattern

```text
created dvr archive for stream='%s' path='%s'
```

## Parameters

- Argument 1 — Stream: Text naming the application and stream.
- Argument 2 — Archive directory: Text giving the DVR storage directory.

## Log examples

Illustrative examples; these are not captured server logs.

```text
created dvr archive for stream='live/camera' path='/data/dvr/live/camera'
```

## Short description

DVR archive registered for recording.

## Trigger condition

Matching stream or application settings are found and a new archive object is loaded and stored.

## Explanation

The archive is returned for use. This path does not check the load result before this message.

## Possible causes

- Matching stream or application settings are found and a new archive object is loaded and stored.

## What to check

1. Check preceding archive-loading errors if the archive is unavailable.

## Recommended action

Check preceding archive-loading errors if the archive is unavailable.
