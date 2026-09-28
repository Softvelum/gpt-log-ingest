# Received OMT video codec is unsupported

Product: Nimble Streamer
Component: OMT media integration
Logging function: log_e
Log level: error
Documentation ID: log_e-bcf7e63ffa8dda0f

## Log pattern

```text
unsupported omt video codec=%d for [%s]
```

## Parameters

- Argument 1 — Codec: Integer OMT codec identifier.
- Argument 2 — OMT name: Requested output name or resolved full input source name.

## Log examples

Illustrative examples; these are not captured server logs.

```text
unsupported omt video codec=0 for [HOST (Camera)]
```

## Short description

Received OMT video codec is unsupported.

## Trigger condition

The receiver supplies video using a codec other than UYVY.

## Explanation

Conversion to an internal media frame returns failure, which stops the current receive operation.

## Possible causes

- The receiver supplies video using a codec other than UYVY.

## What to check

1. Check the codec produced by the selected OMT source.

## Recommended action

Use UYVY for this receive path.
