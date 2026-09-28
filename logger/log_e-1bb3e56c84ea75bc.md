# Transcription connection setup failed

Product: Nimble Streamer
Component: Live speech transcription
Logging function: log_e
Log level: error
Documentation ID: log_e-1bb3e56c84ea75bc

## Log pattern

```text
failed to connect url=%s stream='%s'
```

## Parameters

- Argument 1 — URL: Configured transcription service URL.
- Argument 2 — Stream: Configured stream name.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to connect url=wss://speech.example.net/ stream='live/news'
```

## Short description

Transcription connection setup failed.

## Trigger condition

The new transcription stream fails its service connection setup.

## Explanation

The new stream is not installed and processing of the audio message fails. This can follow unsupported URL protocol or failure to create a websocket connection.

## Possible causes

- The new transcription stream fails its service connection setup.

## What to check

1. Check the URL and preceding connection diagnostics.

## Related log messages

- [Unsupported transcription URL protocol](log_e-d7849aba7245abf4.md)
- [Cannot start transcription websocket connection](log_e-eeeef873bdff5921.md)
