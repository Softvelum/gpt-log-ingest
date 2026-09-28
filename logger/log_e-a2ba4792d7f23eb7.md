# Media input could not restart from the beginning

Product: Nimble Streamer
Component: File media reading
Logging function: log_e
Log level: error
Documentation ID: log_e-a2ba4792d7f23eb7

## Log pattern

```text
failed to seek to begin of %s, error=%d
```

## Parameters

- Argument 1 — File: Input media filename or URL passed to FFmpeg, text.
- Argument 2 — FFmpeg error: Signed integer FFmpeg return code; no unit.

The transcoder file reader repeats at end of input. The server playlist reader also repeats at configured iteration boundaries, but only when its iteration limit permits another pass.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to seek to begin of /srv/media/clip.mp4, error=-5
```

## Short description

Media input could not restart from the beginning.

## Trigger condition

After end of input or a playlist iteration boundary, FFmpeg backward seeking to timestamp zero returns a nonzero result.

## Explanation

Packet reading returns failure and the current repeat cannot continue.

## Possible causes

- The input is not seekable or its demuxer/storage cannot complete the restart seek.

## What to check

1. Check file availability, seek support, and the FFmpeg error.

## Recommended action

Use a readable seekable input or correct the underlying file/demuxer error.
