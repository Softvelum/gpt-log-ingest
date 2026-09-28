# VAAPI output codec is unsupported

Product: Nimble Streamer
Component: Transcoding output management
Logging function: log_e
Log level: error
Documentation ID: log_e-fdf87c551d54d99c

## Log pattern

```text
unsupported vaapi encoder: %s
```

## Parameters

- Argument 1 — Codec: Requested output codec option, text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
unsupported vaapi encoder: unsupported_codec
```

## Short description

VAAPI output codec is unsupported.

## Trigger condition

The selected native VAAPI backend is available, but its requested codec string is outside the accepted set.

## Explanation

Encoder creation fails and the output worker cannot continue.

## Possible causes

- The configured codec is incompatible with this backend; accepted values are h264, plus vp8 when VP8 VAAPI encoding is included in the build.

## What to check

1. Check the backend and exact codec option value.

## Recommended action

Select a supported codec or a backend that provides the required codec.
