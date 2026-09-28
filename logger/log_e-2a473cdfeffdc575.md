# NDI video frame cannot be serialized

Product: Nimble Streamer
Component: Local media output
Logging function: log_e
Log level: error
Documentation ID: log_e-2a473cdfeffdc575

## Log pattern

```text
invalid NDI video message for [%s]
```

## Parameters

- Argument 1 — Output: Local output stream or AJA writer name, text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
invalid NDI video message for [camera]
```

## Short description

NDI video frame cannot be serialized.

## Trigger condition

Creating the outgoing NDI video message fails frame validation, size accounting, or payload writes.

## Explanation

The frame is rejected before writer-state checking and queueing.

## Possible causes

- The frame is missing, dimensions or strides are invalid, planes or caption data are absent, the format is outside YUV420P/NV12/BGRA/UYVY422, or the complete message exceeds 20 megabytes or cannot be buffered.

## What to check

1. Check the frame format, dimensions, plane strides, caption data, and available memory.

## Recommended action

Provide valid supported video frames within the message-size limit.
