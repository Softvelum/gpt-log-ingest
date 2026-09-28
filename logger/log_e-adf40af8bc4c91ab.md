# DVR AVC configuration version is unsupported

Product: Nimble Streamer
Component: DVR recording
Logging function: log_e
Log level: error
Documentation ID: log_e-adf40af8bc4c91ab

## Log pattern

```text
[VM] not supported AVC configuration %d
```

## Parameters

- Argument 1 — Configuration version: Integer AVC decoder configuration version.

## Log examples

Illustrative examples; these are not captured server logs.

```text
[VM] not supported AVC configuration 2
```

## Short description

DVR AVC configuration version is unsupported.

## Trigger condition

The AVC decoder configuration version is not 1.

## Explanation

The sequence header is rejected for this recording pass.

## Possible causes

- The AVC decoder configuration version is not 1.

## What to check

1. Supply a version-1 AVC decoder configuration record.

## Recommended action

Supply a version-1 AVC decoder configuration record.
