# EAC3 decoder is available

Product: Nimble Streamer
Component: Audio decoding
Logging function: log_d
Log level: debug
Documentation ID: log_d-ac6e1b2294bd7f87

## Log pattern

```text
found eac3 decoder for [%s/%s]
```

## Parameters

- Argument 1 — Application: Application name, text.
- Argument 2 — Stream: Stream name, text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
found eac3 decoder for [live/camera]
```

## Short description

EAC3 decoder is available.

## Trigger condition

The first availability lookup finds a registered FFmpeg EAC3 decoder.

## Explanation

Availability is cached and decoder setup continues; this does not yet prove successful packet decoding.
