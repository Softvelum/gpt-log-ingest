# Transcoder input resolution is unavailable

Product: Nimble Streamer
Component: Transcoder orchestration
Logging function: log_e
Log level: error
Documentation ID: log_e-7d2bca0e8acb857d

## Log pattern

```text
failed to get stream resolution for [%s]
```

## Parameters

- Argument 1 — Stream: Input stream name, text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to get stream resolution for [live/camera]
```

## Short description

Transcoder input resolution is unavailable.

## Trigger condition

A configured input has a resolution filter, but its named stream is absent from the available input stream buffers.

## Explanation

Resolution checking fails and these transcoding settings are skipped in this synchronization pass.

## Possible causes

- The required input stream is not present in the set of available decoder streams.

## What to check

1. Check the input application/stream name and whether it is available for transcoding.

## Recommended action

Restore the named input or correct its configured name.
