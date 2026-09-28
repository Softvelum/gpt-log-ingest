# Quick Sync reports changed sequence parameters

Product: Nimble Streamer
Component: Intel Quick Sync
Logging function: log_d
Log level: debug
Documentation ID: log_d-1c3e328dce278aed

## Log pattern

```text
%p seq params changed
```

## Parameters

- Argument 1 — Session: Quick Sync session handle, pointer printed in hexadecimal.

## Log examples

Illustrative examples; these are not captured server logs.

```text
0x12340000 seq params changed
```

## Short description

Quick Sync reports changed sequence parameters.

## Trigger condition

Asynchronous decoding reports the video-parameter-changed warning.

## Explanation

The warning is accepted and processing continues toward synchronization and output using the existing configured output parameters.
