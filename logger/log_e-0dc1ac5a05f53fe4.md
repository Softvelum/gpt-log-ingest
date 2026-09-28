# Audio filtering fell behind its input

Product: Nimble Streamer
Component: Audio filtering worker
Logging function: log_e
Log level: error
Documentation ID: log_e-0dc1ac5a05f53fe4

## Log pattern

```text
failed to filter audio in appropriate rate, id=%s, stream=%s
```

## Parameters

- Argument 1 — Filter identifier: Configured filter identifier, text.
- Argument 2 — Stream: Input stream name, text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to filter audio in appropriate rate, id=audio-main, stream=live/camera
```

## Short description

Audio filtering fell behind its input.

## Trigger condition

Reading a filter feed reports that the consumer is too slow for the retained frame buffer.

## Explanation

Frame processing fails and the audio filter worker exits.

## Possible causes

- The worker has fallen behind the input buffer retention window.

## What to check

1. Check the affected filter and input rate relative to processing capacity.

## Recommended action

Reduce filtering load or restore sufficient processing capacity.
