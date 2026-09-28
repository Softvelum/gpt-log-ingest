# WHEP waits for transcoder registration

Product: Nimble Streamer
Component: WHEP playback helper
Logging function: log_e
Log level: error
Documentation ID: log_e-c29c7acbffc93709

## Log pattern

```text
Please install and register transcoder to start WHEP streaming
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
Please install and register transcoder to start WHEP streaming
```

## Short description

WHEP waits for transcoder registration.

## Trigger condition

The periodic transcoder-availability check has failed and the worker remains in its unavailable interval.

## Explanation

WHEP control processing is suspended while unavailable. Queued/delayed session starts are finished and the managed helper is stopped during cleanup; the notice is limited to roughly once per minute.

## Possible causes

- The server does not report an enabled transcoder.

## What to check

1. Check that the transcoder is installed and registered for this server.

## Recommended action

Install and register the transcoder required for WHEP playback.
