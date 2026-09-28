# RTSP live-stream registration failed

Product: Nimble Streamer
Component: RTSP connections and control
Logging function: log_e
Log level: error
Documentation ID: log_e-2ba289ee07a16ac0

## Log pattern

```text
register stream %s %s failed, client %d
```

## Parameters

- Argument 1 — Application: Live-stream application name.
- Argument 2 — Stream: Live-stream name within its application.
- Argument 3 — Socket: Integer operating-system socket descriptor identifying this connection.

The names identify the output stream for a pulled input, or the requested published stream for an incoming publisher.

## Log examples

Illustrative examples; these are not captured server logs.

```text
register stream live camera failed, client 25
```

## Short description

RTSP live-stream registration failed.

## Trigger condition

The live-stream registration operation returns no stream for the requested application and name.

## Explanation

This occurs after a pulled DESCRIBE response, while accepting an ANNOUNCE, or after asynchronous publishing authorization completes. Each occurrence requests connection shutdown before codec setup.

## Possible causes

- The live-stream registration operation returns no stream for the requested application and name.

## What to check

1. Check whether the requested application and stream can be registered and inspect accompanying registration errors.
