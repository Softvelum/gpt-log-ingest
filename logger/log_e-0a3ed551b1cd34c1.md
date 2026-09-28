# Scenario input frame type is unsupported

Product: Nimble Streamer
Component: Scenario encoding
Logging function: log_e
Log level: error
Documentation ID: log_e-0a3ed551b1cd34c1

## Log pattern

```text
unhandled frame, type=%d for [%s/%s]
```

## Parameters

- Argument 1 — Frame type: Integer internal media-buffer item type.
- Argument 2 — Application: Live-stream application name.
- Argument 3 — Stream: Live-stream name within its application.

## Log examples

Illustrative examples; these are not captured server logs.

```text
unhandled frame, type=255 for [live/camera]
```

## Short description

Scenario input frame type is unsupported.

## Trigger condition

An incoming frame uses a type absent from the scenario handler supported metadata, codec-header, media, subtitle, and cue branches.

## Explanation

Frame processing returns failure; the frame is not delivered through a supported media path.

## Possible causes

- An incoming frame uses a type absent from the scenario handler supported metadata, codec-header, media, subtitle, and cue branches.

## What to check

1. Check the internal frame type supplied for this application and stream.

## Recommended action

Send a frame type supported by the scenario input protocol.
