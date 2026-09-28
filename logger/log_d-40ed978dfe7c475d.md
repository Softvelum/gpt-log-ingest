# DVR archive object created

Product: Nimble Streamer
Component: DVR archive storage
Logging function: log_d
Log level: debug
Documentation ID: log_d-40ed978dfe7c475d

## Log pattern

```text
DvrArchive::DvrArchive stream='%s/%s' path='%s' max_duration=%u segment_duration=%hu
```

## Parameters

- Argument 1 — Application: Text naming the application.
- Argument 2 — Stream: Text naming the stream.
- Argument 3 — Archive directory: Text giving the archive or time-bucket directory.
- Argument 4 — Retention depth: Unsigned configured retention duration in minutes, despite the max_duration label.
- Argument 5 — Segment duration: Unsigned 16-bit target segment duration in seconds.

## Log examples

Illustrative examples; these are not captured server logs.

```text
DvrArchive::DvrArchive stream='live/camera' path='/data/dvr/live/camera/1700000000000' max_duration=60 segment_duration=6
```

## Short description

DVR archive object created.

## Trigger condition

An archive is constructed with its stream, path, retention, and segment settings.

## Explanation

Storage loading or recording follows; this message does not confirm that existing files loaded.

## Possible causes

- An archive is constructed with its stream, path, retention, and segment settings.
