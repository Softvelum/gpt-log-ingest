# FDK AAC support is absent from this build

Product: Nimble Streamer
Component: Transcoding output management
Logging function: log_e
Log level: error
Documentation ID: log_e-ab35df7001dcda91

## Log pattern

```text
unsupported encoder: libfdk_aac for %s/%s
```

## Parameters

- Argument 1 — Application: Output application name, text.
- Argument 2 — Stream: Output stream name, text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
unsupported encoder: libfdk_aac for live/encoded
```

## Short description

FDK AAC support is absent from this build.

## Trigger condition

The FDK AAC encoder is requested in a build compiled without its integration.

## Explanation

Audio encoder creation fails; this branch does not use the default encoder even where the message has informational severity.

## Possible causes

- The running binary lacks this encoder integration.

## What to check

1. Check the chosen audio backend and installed build capabilities.

## Recommended action

Use a compatible available encoder or restore the required build support.
