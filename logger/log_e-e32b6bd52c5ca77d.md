# RTSP authentication could not proceed

Product: Nimble Streamer
Component: RTSP connections and control
Logging function: log_e
Log level: error
Documentation ID: log_e-e32b6bd52c5ca77d

## Log pattern

```text
auth issue on s=%d stream='%s/%s'
```

## Parameters

- Argument 1 — Socket: Integer operating-system socket descriptor identifying this connection.
- Argument 2 — Application: Live-stream application name.
- Argument 3 — Stream: Live-stream name within its application.

## Log examples

Illustrative examples; these are not captured server logs.

```text
auth issue on s=25 stream='live/camera'
```

## Short description

RTSP authentication could not proceed.

## Trigger condition

A 401 response cannot use an allowed first OPTIONS or keepalive retry and is not a DESCRIBE challenge with both credentials and a supported realm.

## Explanation

The client requests shutdown. This also covers a repeated authentication rejection after a retry, missing login or password, or missing supported challenge data.

## Possible causes

- A 401 response cannot use an allowed first OPTIONS or keepalive retry and is not a DESCRIBE challenge with both credentials and a supported realm.

## What to check

1. Check the configured credentials and the authentication challenge returned by the source.

## Recommended action

Provide credentials and a supported Basic or Digest challenge that the source will accept.
