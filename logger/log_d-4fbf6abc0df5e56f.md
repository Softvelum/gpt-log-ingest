# Quick Sync encoder is ready

Product: Nimble Streamer
Component: Intel Quick Sync
Logging function: log_d
Log level: debug
Documentation ID: log_d-4fbf6abc0df5e56f

## Log pattern

```text
%p quick sync encoder created
```

## Parameters

- Argument 1 — Session: Quick Sync session handle, pointer printed in hexadecimal.

## Log examples

Illustrative examples; these are not captured server logs.

```text
0x12340000 quick sync encoder created
```

## Short description

Quick Sync encoder is ready.

## Trigger condition

Encoder initialization has obtained codec parameters and allocated its surface and packet buffers.

## Explanation

The encoder is marked successfully created and can accept frames.
