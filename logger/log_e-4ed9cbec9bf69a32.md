# NVIDIA encoding is absent from this build

Product: Nimble Streamer
Component: Transcoding output management
Logging function: log_e
Log level: error
Documentation ID: log_e-4ed9cbec9bf69a32

## Log pattern

```text
unsupported encoder: nvenc
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
unsupported encoder: nvenc
```

## Short description

NVIDIA encoding is absent from this build.

## Trigger condition

The NVIDIA backend is requested in a build compiled without its integration.

## Explanation

Encoder creation fails and the output worker cannot continue.

## Possible causes

- The selected backend is not supported by the running binary.

## What to check

1. Check the requested encoder and the capabilities of the installed build.

## Recommended action

Use a build containing this backend or select an included encoder.
