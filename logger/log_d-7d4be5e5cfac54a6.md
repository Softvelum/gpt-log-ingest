# DVR retention limits require no segment cleanup

Product: Nimble Streamer
Component: DVR archive maintenance
Logging function: log_d
Log level: debug
Documentation ID: log_d-7d4be5e5cfac54a6

## Log pattern

```text
dvr str='%s/%s' dur=%llu td=%llu sz=%llu tsz=%llu sftd=%llu cleanup skipped
```

## Parameters

- Argument 1 — Application: Text naming the application.
- Argument 2 — Stream: Text naming the stream.
- Argument 3 — Recorded duration: Unsigned total archive duration in milliseconds.
- Argument 4 — Target duration: Unsigned retention depth in milliseconds; the maximum 64-bit value means unlimited.
- Argument 5 — Storage bytes: Unsigned accounted physical media storage size in bytes.
- Argument 6 — Target bytes: Unsigned configured storage limit in bytes; zero means unlimited.
- Argument 7 — Deferred files: Unsigned count of files awaiting physical deletion.

## Log examples

Illustrative examples; these are not captured server logs.

```text
dvr str='live/camera' dur=3600000 td=1800000 sz=104857600 tsz=0 sftd=2 cleanup skipped
```

## Short description

DVR retention limits require no segment cleanup.

## Trigger condition

Recording is within its applicable duration and storage-size limits.

## Explanation

No active segments are deleted, but previously deferred unused files are still checked for removal.

## Possible causes

- Recording is within its applicable duration and storage-size limits.
