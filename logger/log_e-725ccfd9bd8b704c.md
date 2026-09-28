# File media packet could not be read

Product: Nimble Streamer
Component: Audio decoding
Logging function: log_e
Log level: error
Documentation ID: log_e-725ccfd9bd8b704c

## Log pattern

```text
failed to read packet for [%s]
```

## Parameters

- Argument 1 — Stream: Input stream name, text.

Shared by audio and video file-decoder workers. For animated images, the reader also checks replacement of the file at end of input.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to read packet for [live/camera]
```

## Short description

File media packet could not be read.

## Trigger condition

The file stream reader fails to return the next selected-track packet.

## Explanation

The audio or video decoder operation fails and its worker stops.

## Possible causes

- Reading or seeking the file failed; animated-image reopen, metadata access, or cached-packet reference creation may also fail.

## What to check

1. Check the preceding file-read or seek diagnostic and continued file availability. End of file normally triggers a seek and repeat.

## Recommended action

Restore readable, seekable input or correct the file error and restart the pipeline.
