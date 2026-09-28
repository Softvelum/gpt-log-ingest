# Quick Sync frame encoding failed

Product: Nimble Streamer
Component: Intel Quick Sync
Logging function: log_e
Log level: error
Documentation ID: log_e-7cab49bb6132af0a

## Log pattern

```text
%p failed to encode frame %d

```

## Parameters

- Argument 1 — Session: Quick Sync session handle, pointer printed in hexadecimal.
- Argument 2 — Status: Signed integer status returned by the Intel media runtime; no unit.

## Log examples

Illustrative examples; these are not captured server logs.

```text
0x12340000 failed to encode frame -3

```

## Short description

Quick Sync frame encoding failed.

## Trigger condition

Asynchronous encoding returns a status other than success or more-data after device-busy responses have been retried.

## Explanation

The frame-writing operation returns failure before draining encoded output.

## Possible causes

- The media runtime cannot encode the submitted frame with the active session.

## What to check

1. Check the status, input characteristics, and device state; device-busy responses are retried after 500 milliseconds.

## Recommended action

Correct the input or runtime failure and restart encoding.
