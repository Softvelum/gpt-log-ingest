# Subtitle synchronization queue duration is outside its range

Product: Nimble Streamer
Component: Live stream registration and transcription
Logging function: log_e
Log level: error
Documentation ID: log_e-3dbdaa0e169386ca

## Log pattern

```text
invalid [%s] webvtt_sync_queue_duration for [%s/%s]. (should be >=0 && <= 30000)
```

## Parameters

- Argument 1 — transcriber: Transcription provider: whisper, speechmatics, or kwikmotion.
- Argument 2 — application: Application name.
- Argument 3 — stream: Stream name within the application.

## Log examples

Illustrative examples; these are not captured server logs.

```text
invalid [whisper] webvtt_sync_queue_duration for [live/camera]. (should be >=0 && <= 30000)
```

## Short description

Subtitle synchronization queue duration is outside its range.

## Trigger condition

A configured subtitle synchronization duration is converted to a number below zero or above 30,000.

## Explanation

Transcriber initialization returns without assigning the selected request queue. The allowed duration is 0 through 30,000 milliseconds inclusive. This branch checks the converted value, not whether every input character was numeric.

## Possible causes

- The converted queue duration is outside the supported range.

## What to check

1. Check the selected stream and provider subtitle queue duration.

## Recommended action

Set a duration between 0 and 30,000 milliseconds.
