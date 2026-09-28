# NDI output rejects bottom-field-first interlacing

Product: Nimble Streamer
Component: NDI media integration
Logging function: log_e
Log level: error
Documentation ID: log_e-caa9641ddbce8fe4

## Log pattern

```text
ndi only supports tff interlaced frames, add fieldorder=tff into video pipeline, ndi_name="%s"
```

## Parameters

- Argument 1 — NDI name: Requested or resolved NDI source name identifying the affected input or output.

## Log examples

Illustrative examples; these are not captured server logs.

```text
ndi only supports tff interlaced frames, add fieldorder=tff into video pipeline, ndi_name="HOST (Output)"
```

## Short description

NDI output rejects bottom-field-first interlacing.

## Trigger condition

A normal or SEI-timecode NDI sender receives an interlaced frame whose first field is not the top field.

## Explanation

The frame is rejected and video processing returns failure before sending it. Progressive frames and top-field-first interlaced frames use separate accepted paths.

## Possible causes

- A normal or SEI-timecode NDI sender receives an interlaced frame whose first field is not the top field.

## What to check

1. Check the field order of the video supplied to NDI output.

## Recommended action

Set the video pipeline field order to top-field-first using fieldorder=tff.
