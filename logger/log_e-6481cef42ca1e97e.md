# Audio removal is invalid in the current state

Product: Nimble Streamer
Component: Transcoding output management
Logging function: log_e
Log level: error
Documentation ID: log_e-6481cef42ca1e97e

## Log pattern

```text
EncoderThreadStateVideoOnly, failed to remove audio encoder for %s/%s
```

## Parameters

- Argument 1 — Application: Output application name, text.
- Argument 2 — Stream: Output stream name, text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
EncoderThreadStateVideoOnly, failed to remove audio encoder for live/encoded
```

## Short description

Audio removal is invalid in the current state.

## Trigger condition

New settings remove audio while the worker is video-only and uses audio-to-video synchronization.

## Explanation

The settings update fails and the output worker stops.

## Possible causes

- A removal update arrived while this state has no active audio component to remove.

## What to check

1. Check recent source or scenario changes and whether the requested synchronization still has its required reference input.

## Recommended action

Restore consistent input settings and restart the affected output.
