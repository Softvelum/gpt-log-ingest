# Video frame could not pass through the filter

Product: Nimble Streamer
Component: Video filtering
Logging function: log_e
Log level: error
Documentation ID: log_e-dcf3f5f4f996fdd8

## Log pattern

```text
failed to filter video frame, filter=%s, stream=%s
```

## Parameters

- Argument 1 — Filter: Configured filter identifier, text.
- Argument 2 — Input: Input stream name, text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to filter video frame, filter=overlay1, stream=live/overlay
```

## Short description

Video frame could not pass through the filter.

## Trigger condition

Writing an input frame to the filter fails.

## Explanation

The filtering operation fails and the worker stops.

## Possible causes

- Input-context creation failed, frame properties differ without a successful reset, the graph rejects the frame, or output delivery fails.

## What to check

1. Check preceding graph, frame-property, or output-queue messages and the filter expression.

## Recommended action

Correct the specific filter or downstream failure and restart the filter.
