# Requested media track was not found

Product: Nimble Streamer
Component: FFmpeg logging and media input
Logging function: log_e
Log level: error
Documentation ID: log_e-c8c7d17e80dcfe1a

## Log pattern

```text
no stream found, %s stream_id=%d
```

## Parameters

- Argument 1 — Input: Input media name or path, text.
- Argument 2 — Track ordinal: Zero-based ordinal among streams of the requested audio or video type, integer.

## Log examples

Illustrative examples; these are not captured server logs.

```text
no stream found, image.png stream_id=1
```

## Short description

Requested media track was not found.

## Trigger condition

After opening the media, no stream matches the requested media type and ordinal.

## Explanation

The input reader reports failure.

## Possible causes

- The file lacks the requested audio or video track at that ordinal.

## What to check

1. Check the selected media type and zero-based track ordinal.

## Recommended action

Correct the input or requested media setting and retry.
