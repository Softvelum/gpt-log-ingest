# Cannot start transcription websocket connection

Product: Nimble Streamer
Component: Live speech transcription
Logging function: log_e
Log level: error
Documentation ID: log_e-eeeef873bdff5921

## Log pattern

```text
failed to connect transcriber url=%s stream=%s
```

## Parameters

- Argument 1 — URL: Configured service URL.
- Argument 2 — Stream: Configured stream name.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to connect transcriber url=wss://speech.example.net/ stream=live/news
```

## Short description

Cannot start transcription websocket connection.

## Trigger condition

The websocket client cannot create a connection for the configured transcription URL.

## Explanation

The connect attempt returns failure. No active transcription stream is installed by the caller.

## Possible causes

- The websocket client cannot create a connection for the configured transcription URL.

## What to check

1. Check the configured endpoint and websocket connection availability.

## Related log messages

- [Transcription connection setup failed](log_e-1bb3e56c84ea75bc.md)
