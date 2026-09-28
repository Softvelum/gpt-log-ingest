# Received NDI audio format is unsupported

Product: Nimble Streamer
Component: NDI media integration
Logging function: log_e
Log level: error
Documentation ID: log_e-50ddeddc65d76c53

## Log pattern

```text
unsupported ndi audio format: %c%c%c%c
```

## Parameters

- Argument 1 — Format byte 1: Character from bits 0 through 7 of the four-character format identifier.
- Argument 2 — Format byte 2: Character from bits 8 through 15 of the four-character format identifier.
- Argument 3 — Format byte 3: Character from bits 16 through 23 of the four-character format identifier.
- Argument 4 — Format byte 4: Character from bits 24 through 31 of the four-character format identifier.

## Log examples

Illustrative examples; these are not captured server logs.

```text
unsupported ndi audio format: XXXX
```

## Short description

Received NDI audio format is unsupported.

## Trigger condition

An incoming NDI audio frame uses a format other than planar floating-point audio.

## Explanation

The frame cannot be converted into an internal media message, so frame writing returns failure to the receiver.

## Possible causes

- An incoming NDI audio frame uses a format other than planar floating-point audio.

## What to check

1. Check the format returned by the NDI source.

## Recommended action

Supply planar floating-point audio in this NDI receive path.

## Related log messages

- [Received NDI audio frame could not be forwarded](log_e-b7d526b381807a13.md)
