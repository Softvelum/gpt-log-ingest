# Quick Sync encoder synchronization failed

Product: Nimble Streamer
Component: Intel Quick Sync
Logging function: log_e
Log level: error
Documentation ID: log_e-9c0c75d31e3a7d0f

## Log pattern

```text
%p failed to sync encoder %d

```

## Parameters

- Argument 1 — Session: Quick Sync session handle, pointer printed in hexadecimal.
- Argument 2 — Status: Signed integer status returned by the Intel media runtime; no unit.

## Log examples

Illustrative examples; these are not captured server logs.

```text
0x12340000 failed to sync encoder -3

```

## Short description

Quick Sync encoder synchronization failed.

## Trigger condition

Waiting up to 60000 milliseconds for an outstanding encoded packet returns any nonzero status.

## Explanation

The frame-writing operation fails before converting and delivering that packet.

## Possible causes

- The asynchronous encode operation fails or does not complete within the synchronization wait.

## What to check

1. Check the status and device progress or resource pressure.

## Recommended action

Restore device operation and restart the affected encoder.
