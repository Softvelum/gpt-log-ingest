# RTMP publish status command-object placeholder cannot be decoded

Product: Nimble Streamer
Component: RTMP command handling
Logging function: log_e
Log level: error
Documentation ID: log_e-d302cff8cc17dea0

## Log pattern

```text
failed to read null object for publish/onStatus
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to read null object for publish/onStatus
```

## Short description

RTMP publish status command-object placeholder cannot be decoded.

## Trigger condition

The remote publish onStatus response lacks a readable AMF null value at its command-object placeholder.

## Explanation

The pending command response is marked failed and connection processing stops.

## Possible causes

- The remote publish onStatus response lacks a readable AMF null value at its command-object placeholder.

## What to check

1. Inspect the publish status response field order and AMF types.

## Recommended action

Correct the malformed status response at the remote endpoint.
