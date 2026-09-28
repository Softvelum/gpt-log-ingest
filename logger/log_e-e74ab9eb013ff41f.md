# Quick Sync encoded frame buffer could not grow

Product: Nimble Streamer
Component: Intel Quick Sync
Logging function: log_e
Log level: error
Documentation ID: log_e-e74ab9eb013ff41f

## Log pattern

```text
%p failed to alloc quick sync encoder buffer
```

## Parameters

- Argument 1 — Session: Quick Sync session handle, pointer printed in hexadecimal.

## Log examples

Illustrative examples; these are not captured server logs.

```text
0x12340000 failed to alloc quick sync encoder buffer
```

## Short description

Quick Sync encoded frame buffer could not grow.

## Trigger condition

The output assembly buffer cannot reserve space for a NAL unit and its four-byte length prefix.

## Explanation

NAL assembly fails and the encoded frame is not delivered.

## Possible causes

- The output assembly buffer cannot satisfy the requested capacity or allocation.

## What to check

1. Check memory availability and unusually large encoded frames.

## Recommended action

Restore buffer capacity or reduce encoded frame demand and restart the stream.
