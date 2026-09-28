# Remote RTMP server returns an error

Product: Nimble Streamer
Component: RTMP command handling
Logging function: log_e
Log level: error
Documentation ID: log_e-eb838b34db7fe935

## Log pattern

```text
_error response received for '%s'
```

## Parameters

- Argument 1 — Stream URL: RTMP or RTMPS URL identifying the outbound stream connection.

## Log examples

Illustrative examples; these are not captured server logs.

```text
_error response received for 'rtmp://destination.example:1935/live/camera'
```

## Short description

Remote RTMP server returns an error.

## Trigger condition

An outbound pull receives _error, or an outbound publisher receives _error outside its handled Adobe or Limelight challenge exchange.

## Explanation

The remote error causes connection processing to fail. This log gives the outbound stream URL but does not decode the server error details.

## Possible causes

- An outbound pull receives _error, or an outbound publisher receives _error outside its handled Adobe or Limelight challenge exchange.

## What to check

1. Check the remote server error response and the pull or publish destination for the named stream.

## Recommended action

Resolve the rejection reported by the remote server before retrying.
