# Live JPEG thumbnail creation failed

Product: Nimble Streamer
Component: JPEG thumbnails
Logging function: log_e
Log level: error
Documentation ID: log_e-1a52dbd8cfdae4ca

## Log pattern

```text
failed to create jpg thumbnail for stream='%s/%s'
```

## Parameters

- Argument 1 — Application: Application name, text.
- Argument 2 — Stream: Stream name, text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to create jpg thumbnail for stream='live/camera'
```

## Short description

Live JPEG thumbnail creation failed.

## Trigger condition

A live thumbnail request cannot complete decoding, JPEG production or storage in its result buffer.

## Explanation

No new thumbnail is installed for the stream from this request; the worker continues with later requests.

## Possible causes

- The input codec is not H.264, HEVC or AV1.
- Decoding or JPEG creation fails, no image appears after flushing, or the thumbnail buffer cannot accept the result.

## What to check

1. Check the live video codec, supplied frames, requested thumbnail dimensions and nearby decoder errors.

## Recommended action

Provide valid supported video and correct the reported conversion or resource failure.
