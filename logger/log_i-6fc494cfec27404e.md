# Faulting local media mapping was closed

Product: Nimble Streamer
Component: VOD transmuxing
Logging function: log_i
Log level: info
Documentation ID: log_i-6fc494cfec27404e

## Log pattern

```text
closed r->file_name='%s'
```

## Parameters

- Argument 1 — Media filename: Text filesystem path of the source media.

## Log examples

Illustrative examples; these are not captured server logs.

```text
closed r->file_name='/media/movie.mp4'
```

## Short description

Faulting local media mapping was closed.

## Trigger condition

Bus-signal recovery has called the unmap-and-close operation and still has the active request.

## Explanation

The named source is removed from the opened-media cache and its media object is deleted.

## Possible causes

- Bus-signal recovery has called the unmap-and-close operation and still has the active request.

## What to check

1. Investigate the preceding bus signal and source-file changes before retrying this media.

## Recommended action

Investigate the preceding bus signal and source-file changes before retrying this media.
