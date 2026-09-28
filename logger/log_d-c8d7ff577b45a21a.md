# Local media or DVR file open failed

Product: Nimble Streamer
Component: HTTP client sessions
Logging function: log_d
Log level: debug
Documentation ID: log_d-c8d7ff577b45a21a

## Log pattern

```text
failed to open file='%s' errno=%d
```

## Parameters

- Argument 1 — Filename: Text filesystem path of the local media or DVR archive file.
- Argument 2 — File error: Integer operating-system error number captured after the failed open.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to open file='/media/movie.mp4' errno=2
```

## Short description

Local media or DVR file open failed.

## Trigger condition

Opening the requested local file or DVR archive for reading returns a failure.

## Explanation

HTTP 404 is prepared; no file data is sent.

## Possible causes

- Opening the requested local file or DVR archive for reading returns a failure.

## What to check

1. Check the exact path, file existence, service read permissions, and the reported operating-system error.

## Recommended action

Check the exact path, file existence, service read permissions, and the reported operating-system error.
