# SDI audio channel order is unsupported

Product: Nimble Streamer
Component: SDI device input
Logging function: log_e
Log level: error
Documentation ID: log_e-565dbde8edfa9fd8

## Log pattern

```text
unsupported channel layout for [%s]
```

## Parameters

- Argument 1 — Stream: Input stream name, text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
unsupported channel layout for [live/camera]
```

## Short description

SDI audio channel order is unsupported.

## Trigger condition

The first decoded device audio frame uses a channel order other than native or unspecified.

## Explanation

Audio decoding returns failure; an unspecified layout would instead receive the default layout for its channel count.

## Possible causes

- The device decoder supplies a channel order this input path does not support.

## What to check

1. Check the decoded audio channel order and device audio configuration.

## Recommended action

Select audio with a native channel layout.
