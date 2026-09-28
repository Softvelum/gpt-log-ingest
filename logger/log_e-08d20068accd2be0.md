# DVR JPEG thumbnail request was refused

Product: Nimble Streamer
Component: DVR recording
Logging function: log_e
Log level: error
Documentation ID: log_e-08d20068accd2be0

## Log pattern

```text
failed to request jpg thumbnail generation for stream='%s'
```

## Parameters

- Argument 1 — Stream: Text naming the application and stream.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to request jpg thumbnail generation for stream='live/camera'
```

## Short description

DVR JPEG thumbnail request was refused.

## Trigger condition

A thumbnail request was produced but the server refuses it while shutdown is not requested.

## Explanation

Recording continues without resetting the archive.

## Possible causes

- A thumbnail request was produced but the server refuses it while shutdown is not requested.

## What to check

1. Check thumbnail processing capacity and preceding thumbnail errors.

## Recommended action

Check thumbnail processing capacity and preceding thumbnail errors.
