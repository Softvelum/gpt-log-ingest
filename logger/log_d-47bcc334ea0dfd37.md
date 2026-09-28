# OMT audio waits for a video timecode reference

Product: Nimble Streamer
Component: OMT media integration
Logging function: log_d
Log level: debug
Documentation ID: log_d-47bcc334ea0dfd37

## Log pattern

```text
skip audio frame till first video frame with sei timecode for [%s]
```

## Parameters

- Argument 1 — OMT name: Requested output name or resolved full input source name.

## Log examples

Illustrative examples; these are not captured server logs.

```text
skip audio frame till first video frame with sei timecode for [Output]
```

## Short description

OMT audio waits for a video timecode reference.

## Trigger condition

SEI-timecode output receives audio without both a valid system-time reference and its video timestamp.

## Explanation

The frame is skipped and the message handler reports success. Ordinary unscheduled OMT sending bypasses this check.

## Possible causes

- SEI-timecode output receives audio without both a valid system-time reference and its video timestamp.

## What to check

1. Check that the source supplies SEI timecode and a video timestamp.

## Recommended action

Supply a video timecode reference before scheduled audio output.

## Related log messages

- [OMT audio timestamp cannot use the video timecode reference](log_e-b99c25ebbe70f0ea.md)
