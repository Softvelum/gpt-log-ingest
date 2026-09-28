# RTMP stream registration fails

Product: Nimble Streamer
Component: RTMP command handling
Logging function: log_e
Log level: error
Documentation ID: log_e-71b0ffa43622ad4a

## Log pattern

```text
failed to register stream '%s/%s'
```

## Parameters

- Argument 1 — Application: Application receiving the publish request.
- Argument 2 — Stream: Stream name being registered for the publisher.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to register stream 'live/camera'
```

## Short description

RTMP stream registration fails.

## Trigger condition

An authorized or provisionally accepted publish request cannot register its stream as a live input.

## Explanation

The publish command fails before acknowledging successful publishing, and connection processing stops.

## Possible causes

- An authorized or provisionally accepted publish request cannot register its stream as a live input.

## What to check

1. Check other stream-registration logs and the current publisher for the same application and stream.

## Recommended action

Resolve the registration failure before retrying publication.
