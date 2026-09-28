# Quick Sync software encoder selected

Product: Nimble Streamer
Component: Intel Quick Sync
Logging function: log_d
Log level: debug
Documentation ID: log_d-73db3aaab253bd19

## Log pattern

```text
%p quick sync %d.%d sw
```

## Parameters

- Argument 1 — Session: Quick Sync session handle, pointer printed in hexadecimal.
- Argument 2 — Major version: Selected media API major version, integer.
- Argument 3 — Minor version: Selected media API minor version, integer.

## Log examples

Illustrative examples; these are not captured server logs.

```text
0x12340000 quick sync 1.17 sw
```

## Short description

Quick Sync software encoder selected.

## Trigger condition

Encoder session setup identifies the software implementation.

## Explanation

The selected API version is reported and encoder configuration continues.
