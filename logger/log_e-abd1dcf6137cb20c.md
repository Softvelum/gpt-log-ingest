# AVC decoder configuration is truncated

Product: Nimble Streamer
Component: Live stream codec and packaging setup
Logging function: log_e
Log level: error
Documentation ID: log_e-abd1dcf6137cb20c

## Log pattern

```text
[VM] wrong message size %d, [%s/%s]
```

## Parameters

- Argument 1 — Configuration size: Total AVC decoder-configuration byte count.
- Argument 2 — Application: Application containing the affected live stream.
- Argument 3 — Stream: Live stream name.

## Log examples

Illustrative examples; these are not captured server logs.

```text
[VM] wrong message size 5, [live/camera]
```

## Short description

AVC decoder configuration is truncated.

## Trigger condition

The AVC configuration is shorter than its fixed fields or lacks a declared sequence/picture parameter-set length or payload.

## Explanation

An empty transport header is ensured and AVC setup fails. The printed size is the entire configuration size, not the number of missing bytes.

## Possible causes

- The AVC configuration is shorter than its fixed fields or lacks a declared sequence/picture parameter-set length or payload.

## What to check

1. Compare declared parameter-set lengths and counts with the received configuration.

## Recommended action

Send a complete AVC decoder configuration.
