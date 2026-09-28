# DVR AVC path received another codec

Product: Nimble Streamer
Component: DVR recording
Logging function: log_e
Log level: error
Documentation ID: log_e-8a49faba10ccb89b

## Log pattern

```text
[VM] wrong codec %d. only AVC codec supported
```

## Parameters

- Argument 1 — Codec ID: Integer codec ID from the low four bits of the video tag.

## Log examples

Illustrative examples; these are not captured server logs.

```text
[VM] wrong codec 2. only AVC codec supported
```

## Short description

DVR AVC path received another codec.

## Trigger condition

The legacy video tag codec ID is not AVC.

## Explanation

This AVC recording operation returns failure.

## Possible causes

- The legacy video tag codec ID is not AVC.

## What to check

1. Check the incoming codec tag and video processing path.

## Recommended action

Check the incoming codec tag and video processing path.
