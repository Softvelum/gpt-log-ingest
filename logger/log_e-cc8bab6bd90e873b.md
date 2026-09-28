# NVIDIA encoder has no free frame buffer

Product: Nimble Streamer
Component: NVIDIA video encoding
Logging function: log_e
Log level: error
Documentation ID: log_e-cc8bab6bd90e873b

## Log pattern

```text
failed to get free buffer encoder=%p
```

## Parameters

- Argument 1 — Encoder: NVIDIA encoder session handle, pointer printed in hexadecimal.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to get free buffer encoder=0x12340000
```

## Short description

NVIDIA encoder has no free frame buffer.

## Trigger condition

Every slot in the initialized encoder buffer pool is marked in use.

## Explanation

The current frame write returns failure before allocating or filling an input buffer.

## Possible causes

- Previously submitted pictures have not freed enough buffer slots.

## What to check

1. Check preceding delayed-output, bitstream-lock and encoding errors.

## Recommended action

Restore output draining or reduce encoder delay/load before retrying.
