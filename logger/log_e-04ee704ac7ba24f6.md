# Media packet read failed

Product: Nimble Streamer
Component: File media reading
Logging function: log_e
Log level: error
Documentation ID: log_e-04ee704ac7ba24f6

## Log pattern

```text
failed to read pkt from %s, error=%d
```

## Parameters

- Argument 1 — File: Input media filename or URL passed to FFmpeg, text.
- Argument 2 — FFmpeg error: Signed integer FFmpeg return code; no unit.

Shared by the looping transcoder file reader and server playlist VOD reader. A playlist reader that reaches its configured iteration limit exits earlier without this error.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to read pkt from /srv/media/clip.mp4, error=-5
```

## Short description

Media packet read failed.

## Trigger condition

FFmpeg packet reading returns a nonzero result after any end-of-input restart attempt.

## Explanation

The reader returns failure without delivering another selected-track packet.

## Possible causes

- The input is unavailable, damaged, or unreadable, or a restarted input still returns end of file.

## What to check

1. Check the FFmpeg error and input availability; ordinary first end-of-file results are handled by seeking before this check.

## Recommended action

Restore valid readable media and restart the affected playback or transcoding input.
