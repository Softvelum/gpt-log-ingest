# DVR archive reload is already in progress

Product: Nimble Streamer
Component: DVR
Logging function: log_i
Log level: info
Documentation ID: log_i-ea39de6ff230fd02

## Log pattern

```text
DVR archive='%s' loading is in progress
```

## Parameters

- Argument 1 — Stream: Text naming the application and stream.

## Log examples

Illustrative examples; these are not captured server logs.

```text
DVR archive='live/camera' loading is in progress
```

## Short description

DVR archive reload is already in progress.

## Trigger condition

A single-archive reload request finds the archive in a nonready state.

## Explanation

The management operation returns busy.

## Possible causes

- A single-archive reload request finds the archive in a nonready state.

## What to check

1. Wait for the current archive load to complete before retrying.

## Recommended action

Wait for the current archive load to complete before retrying.
