# DVR export archive is not registered

Product: Nimble Streamer
Component: DVR
Logging function: log_e
Log level: error
Documentation ID: log_e-f8bf58c6bc9ac787

## Log pattern

```text
failed to find app_stream '%s' in DVR archives
```

## Parameters

- Argument 1 — Stream: Text naming the application and stream.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to find app_stream 'live/camera' in DVR archives
```

## Short description

DVR export archive is not registered.

## Trigger condition

A download-segment lookup finds no archive for its stream.

## Explanation

Segment lookup fails.

## Possible causes

- A download-segment lookup finds no archive for its stream.

## What to check

1. Check the export stream name and attached DVR settings.

## Recommended action

Check the export stream name and attached DVR settings.
