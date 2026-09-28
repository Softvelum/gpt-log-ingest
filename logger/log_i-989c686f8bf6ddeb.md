# Transcription removed after websocket closure

Product: Nimble Streamer
Component: Live speech transcription
Logging function: log_i
Log level: info
Documentation ID: log_i-989c686f8bf6ddeb

## Log pattern

```text
removed transcriber on connection %s stream='%s'
```

## Parameters

- Argument 1 — Event: Either error or close, identifying the websocket callback event.
- Argument 2 — Stream: Configured stream name.

## Log examples

Illustrative examples; these are not captured server logs.

```text
removed transcriber on connection error stream='live/news'
```

## Short description

Transcription removed after websocket closure.

## Trigger condition

The websocket reports connection error or closure for an installed transcription stream.

## Explanation

The transcription stream and its websocket-context association are removed. Later audio can initiate a new connection subject to the restart delay.

## Possible causes

- The websocket reports connection error or closure for an installed transcription stream.

## What to check

1. Check whether the reported event is error or close and inspect nearby connection messages.

## Related log messages

- [Transcription connection setup failed](log_e-1bb3e56c84ea75bc.md)
- [Transcription stream created](log_i-1b137b91d785896d.md)
