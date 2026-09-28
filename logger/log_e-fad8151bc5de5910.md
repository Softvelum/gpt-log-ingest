# VAAPI input surface is not ready

Product: Nimble Streamer
Component: VAAPI video encoding
Logging function: log_e
Log level: error
Documentation ID: log_e-fad8151bc5de5910

## Log pattern

```text
%p bad surface status=%d
```

## Parameters

- Argument 1 — Encoder: Encoder instance address, pointer printed in hexadecimal.
- Argument 2 — Surface state: VAAPI surface-status enumeration, integer; no unit.

## Log examples

Illustrative examples; these are not captured server logs.

```text
0x12340000 bad surface status=1
```

## Short description

VAAPI input surface is not ready.

## Trigger condition

The surface-status query succeeds but reports a status other than ready.

## Explanation

The input frame is not copied and frame encoding fails.

## Possible causes

- The selected surface is still busy or otherwise unavailable.

## What to check

1. Check the reported surface state and preceding encoding or synchronization results.

## Recommended action

Resolve stalled device work and restart the encoder.
