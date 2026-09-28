# Received NDI video frame could not be forwarded

Product: Nimble Streamer
Component: NDI media integration
Logging function: log_e
Log level: error
Documentation ID: log_e-985cce328d0b1a8c

## Log pattern

```text
failed to write ndi video frame, ndi_name=%s
```

## Parameters

- Argument 1 — NDI name: Requested or resolved NDI source name identifying the affected input or output.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to write ndi video frame, ndi_name=HOST (Camera)
```

## Short description

Received NDI video frame could not be forwarded.

## Trigger condition

Converting or delivering a captured NDI video frame returns failure.

## Explanation

The captured NDI frame is released and the receive operation returns failure. Possible failing stages include unsupported pixel format, invalid or oversized frame metadata, allocation, message construction, or parent-process delivery.

## Possible causes

- Converting or delivering a captured NDI video frame returns failure.

## What to check

1. Check preceding format messages and the parent-process frame-delivery connection.

## Related log messages

- [Received NDI video format is unsupported](log_e-d7f82cbee65e4781.md)
