# Could not create the RTSP or SLDP playback worker

Product: Nimble Streamer
Component: Server startup and services
Logging function: log_e
Log level: error
Documentation ID: log_e-9eba421c40edf0de

## Log pattern

```text
failed to create RTSP sender thread
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to create RTSP sender thread
```

## Short description

Could not create the RTSP or SLDP playback worker.

## Trigger condition

A configured RTSP playback worker or a configured WebSocket live playback worker is being created. The WebSocket occurrence reuses the RTSP wording. Thread creation reports failure.

## Explanation

The remaining server startup sequence is skipped and the server run routine returns failure. This path does not reach the normal worker-join sequence. The preceding thread resource error can provide the underlying reason.

## Possible causes

- Creation of the RTSP or SLDP playback worker failed.

## What to check

1. Inspect the preceding thread resource error for the failed RTSP or SLDP playback worker.

## Related log messages

- [Server worker thread could not start](log_e-7e7eeb5817c6f070.md)
