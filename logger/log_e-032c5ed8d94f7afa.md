# DVB bitmap subtitle decoder unavailable

Product: Nimble Streamer
Component: Caption and subtitle handling
Logging function: log_e
Log level: error
Documentation ID: log_e-032c5ed8d94f7afa

## Log pattern

```text
failed to find dvb subtitle decoder for [%s]
```

## Parameters

- Argument 1 — Stream: Application and stream name.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to find dvb subtitle decoder for [live/news]
```

## Short description

DVB bitmap subtitle decoder unavailable.

## Trigger condition

The media library cannot find a DVB subtitle decoder.

## Explanation

Decoder initialization fails; the current subtitle decode returns failure.

## Possible causes

- The media library cannot find a DVB subtitle decoder.

## What to check

1. Check that the loaded media library includes DVB subtitle decoding.
