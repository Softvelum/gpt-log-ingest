# DVR append rollback could not truncate its file

Product: Nimble Streamer
Component: DVR archive storage
Logging function: log_e
Log level: error
Documentation ID: log_e-ecd808fd7e71a731

## Log pattern

```text
failed to truncate segment file='%s' size=%d error=%d message='%s'
```

## Parameters

- Argument 1 — Storage file: Text naming the recorded media storage file.
- Argument 2 — Earlier file size: Wide integer file size captured before the append; this is not necessarily the requested truncation offset.
- Argument 3 — Filesystem error: Integer filesystem error code.
- Argument 4 — Error message: Text describing the reported filesystem failure.

A wide file-size value is supplied to a 32-bit integer conversion.

## Log examples

No sound example is given because the file-size argument does not match its printf conversion.

## Short description

DVR append rollback could not truncate its file.

## Trigger condition

Writing the segment database record failed and truncating the media file back to its prior offset also fails.

## Explanation

The segment is not registered; size accounting is adjusted even though file truncation failed.

## Possible causes

- Writing the segment database record failed and truncating the media file back to its prior offset also fails.

## What to check

1. Check both database-write and media-file truncation errors.

## Recommended action

Check both database-write and media-file truncation errors.
