# Whisper transcription is missing its model path

Product: Nimble Streamer
Component: Live stream registration and transcription
Logging function: log_e
Log level: error
Documentation ID: log_e-6bd7b40de30d495f

## Log pattern

```text
whisper_model_path is empty for [%s/%s]
```

## Parameters

- Argument 1 — application: Application name.
- Argument 2 — stream: Stream name within the application.

## Log examples

Illustrative examples; these are not captured server logs.

```text
whisper_model_path is empty for [live/camera]
```

## Short description

Whisper transcription is missing its model path.

## Trigger condition

The Whisper provider was selected but its model path is empty after loading stream parameters.

## Explanation

Transcriber initialization returns without assigning the provider request queue. This diagnostic is available only in builds with the corresponding transcription support; live stream registration still continues.

## Possible causes

- The effective Whisper model path is empty.

## What to check

1. Check the provider settings applied to this application and stream.

## Recommended action

Supply the required model path.
