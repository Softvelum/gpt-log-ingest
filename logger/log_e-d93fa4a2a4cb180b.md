# AAC explicit-frequency configuration is unsupported

Product: Nimble Streamer
Component: Live stream codec and packaging setup
Logging function: log_e
Log level: error
Documentation ID: log_e-d93fa4a2a4cb180b

## Log pattern

```text
[AM] wrong samplingFrequencyIndex %d for [%s/%s]
```

## Parameters

- Argument 1 — Frequency index: Rejected AAC sampling-frequency index; this branch prints 15.
- Argument 2 — Application: Application containing the affected live stream.
- Argument 3 — Stream: Live stream name.

## Log examples

Illustrative examples; these are not captured server logs.

```text
[AM] wrong samplingFrequencyIndex 15 for [live/camera]
```

## Short description

AAC explicit-frequency configuration is unsupported.

## Trigger condition

The AAC sampling-frequency index is 15, indicating the explicit-frequency form.

## Explanation

AAC codec setup returns failure instead of accepting that configuration.

## Possible causes

- The AAC sampling-frequency index is 15, indicating the explicit-frequency form.

## What to check

1. Check the sampling-frequency index in the AAC decoder configuration.

## Recommended action

Use a supported indexed sample-rate configuration.
