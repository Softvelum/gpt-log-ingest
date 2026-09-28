# Quick Sync hardware decoder selected

Product: Nimble Streamer
Component: Intel Quick Sync
Logging function: log_i
Log level: info
Documentation ID: log_i-7dc9728b07aeeb1e

## Log pattern

```text
%p quick sync %d.%d hw
```

## Parameters

- Argument 1 — Session: Quick Sync session handle, pointer printed in hexadecimal.
- Argument 2 — Major version: Selected media API major version, integer.
- Argument 3 — Minor version: Selected media API minor version, integer.

## Log examples

Illustrative examples; these are not captured server logs.

```text
0x12340000 quick sync 1.17 hw
```

## Short description

Quick Sync hardware decoder selected.

## Trigger condition

Decoder setup recognizes a hardware implementation; on Linux the VA display has also been opened and bound successfully.

## Explanation

The selected media API version is reported and decoder setup continues.
