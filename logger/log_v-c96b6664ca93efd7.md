# Transport stream publishing starts

Product: Nimble Streamer
Component: MPEG-TS stream registration and parsing
Logging function: log_v
Log level: verbose
Documentation ID: log_v-c96b6664ca93efd7

## Log pattern

```text
start publishing stream='%s/%s' %s
```

## Parameters

- Argument 1 — Application: Source application.
- Argument 2 — Stream: Source stream.
- Argument 3 — Publishing context: Output destination and mode description, including SRT peer/group information where applicable.

## Log examples

Illustrative examples; these are not captured server logs.

```text
start publishing stream='live/camera' to 'srt://192.0.2.20:5000' mode='push'
```

## Short description

Transport stream publishing starts.

## Trigger condition

A media item is ready for a stream whose output has publishing context but whose stream iterator has not yet recorded that context.

## Explanation

Publishing context is saved for a later stop message. This line precedes packaging the selected media item.

## Possible causes

- A media item is ready for a stream whose output has publishing context but whose stream iterator has not yet recorded that context.

## What to check

1. Match the application, stream, and destination/mode context with expected output lifecycle changes.

## Recommended action

No action is needed for expected publishing lifecycle changes.
