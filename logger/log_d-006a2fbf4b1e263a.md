# Transcription service message received

Product: Nimble Streamer
Component: Live speech transcription
Logging function: log_d
Log level: debug
Documentation ID: log_d-006a2fbf4b1e263a

## Log pattern

```text
recv msg=%s for stream='%s'
```

## Parameters

- Argument 1 — Message: Complete received JSON message.
- Argument 2 — Stream: Configured stream name.

## Log examples

Illustrative examples; these are not captured server logs.

```text
recv msg={"message":"RecognitionStarted"} for stream='live/news'
```

## Short description

Transcription service message received.

## Trigger condition

The received service JSON message has a message type other than Error or Info.

## Explanation

The complete JSON response is logged at the corresponding severity. Transcription and translation responses may subsequently generate subtitles; this log alone does not close the connection.

## Possible causes

- The received service JSON message has a message type other than Error or Info.

## What to check

1. Inspect the message type and any service-provided detail in the response.
