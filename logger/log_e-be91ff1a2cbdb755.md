# RTSP DESCRIBE supplied no media descriptions

Product: Nimble Streamer
Component: RTSP connections and control
Logging function: log_e
Log level: error
Documentation ID: log_e-be91ff1a2cbdb755

## Log pattern

```text
unable to get streams from client: %d
```

## Parameters

- Argument 1 — Socket: Integer operating-system socket descriptor identifying this connection.

## Log examples

Illustrative examples; these are not captured server logs.

```text
unable to get streams from client: 25
```

## Short description

RTSP DESCRIBE supplied no media descriptions.

## Trigger condition

The DESCRIBE response produces an empty list of parsed SDP media tracks.

## Explanation

The client shuts down before registering or setting up any track.

## Possible causes

- The DESCRIBE response produces an empty list of parsed SDP media tracks.

## What to check

1. Check the DESCRIBE response body for parseable SDP media descriptions.

## Recommended action

Supply an SDP description containing the intended media tracks.

## Related log messages

- [RTSP input has no usable media tracks](log_e-f5f37665de480ac8.md)
