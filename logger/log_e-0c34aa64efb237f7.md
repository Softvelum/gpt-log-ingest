# libvpx encoding is absent from this build

Product: Nimble Streamer
Component: Transcoding output management
Logging function: log_e
Log level: error
Documentation ID: log_e-0c34aa64efb237f7

## Log pattern

```text
unsupported encoder: libvpx
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
unsupported encoder: libvpx
```

## Short description

libvpx encoding is absent from this build.

## Trigger condition

The libvpx backend is requested in a build compiled without its integration.

## Explanation

Encoder creation fails and the output worker cannot continue.

## Possible causes

- The selected backend is not supported by the running binary.

## What to check

1. Check the requested encoder and the capabilities of the installed build.

## Recommended action

Use a build containing this backend or select an included encoder.
