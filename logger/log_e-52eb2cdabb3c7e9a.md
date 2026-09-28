# DVR archive load could not complete

Product: Nimble Streamer
Component: DVR archive storage
Logging function: log_e
Log level: error
Documentation ID: log_e-52eb2cdabb3c7e9a

## Log pattern

```text
failed to load dvr archive app='%s' stream='%s' path='%s'
```

## Parameters

- Argument 1 — Application: Text naming the application.
- Argument 2 — Stream: Text naming the stream.
- Argument 3 — Archive directory: Text giving the archive or time-bucket directory.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to load dvr archive app='live' stream='camera' path='/data/dvr/live/camera/1700000000000'
```

## Short description

DVR archive load could not complete.

## Trigger condition

Loading encounters unreadable or invalid subtitle metadata or an unsupported loaded segment category.

## Explanation

Loading returns failure without marking the archive ready.

## Possible causes

- Loading encounters unreadable or invalid subtitle metadata or an unsupported loaded segment category.

## What to check

1. Check recorded subtitle files, language/descriptor metadata, and the segment database.

## Recommended action

Check recorded subtitle files, language/descriptor metadata, and the segment database.
