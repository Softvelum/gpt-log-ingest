# Received OMT audio codec is unsupported

Product: Nimble Streamer
Component: OMT media integration
Logging function: log_e
Log level: error
Documentation ID: log_e-acaf8afc2d096b82

## Log pattern

```text
unsupported omt audio codec=%d for [%s]
```

## Parameters

- Argument 1 — Codec: Integer OMT codec identifier.
- Argument 2 — OMT name: Requested output name or resolved full input source name.

## Log examples

Illustrative examples; these are not captured server logs.

```text
unsupported omt audio codec=0 for [HOST (Camera)]
```

## Short description

Received OMT audio codec is unsupported.

## Trigger condition

The receiver supplies audio using a codec other than FPA1 planar floating-point audio.

## Explanation

Conversion to an internal media frame returns failure, which stops the current receive operation.

## Possible causes

- The receiver supplies audio using a codec other than FPA1 planar floating-point audio.

## What to check

1. Check the codec produced by the selected OMT source.

## Recommended action

Use FPA1 planar floating-point audio for this receive path.
