# FLAC packet decoding failed

Product: Nimble Streamer
Component: Audio decoding
Logging function: log_e
Log level: error
Documentation ID: log_e-2ac04568c9d0f7ea

## Log pattern

```text
failed to decode FLAC packet from [%s/%s]
```

## Parameters

- Argument 1 — Application: Application name, text.
- Argument 2 — Stream: Stream name, text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to decode FLAC packet from [live/camera]
```

## Short description

FLAC packet decoding failed.

## Trigger condition

The selected decoder returns failure while processing a FLAC packet from the live input buffer.

## Explanation

The audio item fails and the decoder worker leaves its processing loop.

## Possible causes

- The decoder rejects the payload, cannot initialize or allocate output, or its output delivery fails.

## What to check

1. Check preceding codec or output-queue diagnostics and the input packet or codec configuration.

## Recommended action

Correct the specific decoder or output failure and restart the affected audio pipeline.
