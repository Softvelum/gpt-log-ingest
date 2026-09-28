# Remote VOD source cache reservation failed

Product: Nimble Streamer
Component: HTTP client sessions
Logging function: log_e
Log level: error
Documentation ID: log_e-4769dbfcce6a847b

## Log pattern

```text
failed to reserve cache item='%s'
```

## Parameters

- Argument 1 — Source cache key: Text cache key for the remote VOD input file.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to reserve cache item='/vod/movie.mp4'
```

## Short description

Remote VOD source cache reservation failed.

## Trigger condition

No source cache item is returned while reserving or locating the remote VOD input.

## Explanation

HTTP 500 is prepared because source download or transmuxing cannot proceed.

## Possible causes

- No source cache item is returned while reserving or locating the remote VOD input.

## What to check

1. Check adjacent cache reservation and memory errors, and confirm the remote VOD request maps to the intended source.

## Recommended action

Check adjacent cache reservation and memory errors, and confirm the remote VOD request maps to the intended source.
