# Single DVR archive reload started

Product: Nimble Streamer
Component: DVR
Logging function: log_i
Log level: info
Documentation ID: log_i-50d1b13b6d3ae1cd

## Log pattern

```text
Reloading DVR archive='%s'
```

## Parameters

- Argument 1 — Stream: Text naming the application and stream.

## Log examples

Illustrative examples; these are not captured server logs.

```text
Reloading DVR archive='live/camera'
```

## Short description

Single DVR archive reload started.

## Trigger condition

A management reload finds an existing ready archive and batch loading is complete.

## Explanation

A forced archive reload starts.

## Possible causes

- A management reload finds an existing ready archive and batch loading is complete.
