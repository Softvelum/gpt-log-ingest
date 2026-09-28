# NVIDIA video parser creation failed

Product: Nimble Streamer
Component: NVIDIA video decoding
Logging function: log_e
Log level: error
Documentation ID: log_e-37b1db3b0b8feb86

## Log pattern

```text
failed to create nv decoder for [%s], res=%d
```

## Parameters

- Argument 1 — Stream: Input stream name, text.
- Argument 2 — CUDA status: CUDA or NVIDIA video-decoder driver result, integer; no unit.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to create nv decoder for [live/camera], res=2
```

## Short description

NVIDIA video parser creation failed.

## Trigger condition

Creating the NVIDIA video parser with the selected codec and time scale returns a driver error.

## Explanation

Decoder input initialization fails before packet parsing. The message names a decoder, but the failed operation creates its parser.

## Possible causes

- The driver cannot initialize a parser for the requested codec and parameters.

## What to check

1. Check the result code, codec support and GPU driver installation.

## Recommended action

Correct the parser setup or driver support.
