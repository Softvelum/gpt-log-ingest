# Cannot set SRT stream ID

Product: Nimble Streamer
Component: SRT transport
Logging function: log_e
Log level: error
Documentation ID: log_e-a33cfe4bb03f1bd1

## Log pattern

```text
failed to set streamid=%s, s=%d
```

## Parameters

- Argument 1 — Stream ID: Configured SRT stream-ID string.
- Argument 2 — Socket: SRT socket or group identifier.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to set streamid=live/news, s=100
```

## Short description

Cannot set SRT stream ID.

## Trigger condition

The SRT library rejects the configured stream identifier.

## Explanation

Socket settings application stops with failure before connection establishment.

## Possible causes

- The SRT library rejects the configured stream identifier.

## What to check

1. Check the configured stream ID and socket state.
