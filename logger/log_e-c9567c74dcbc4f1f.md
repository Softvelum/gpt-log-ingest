# HLS body has an unsupported encryption state

Product: Nimble Streamer
Component: HLS and MPEG-TS HTTP input
Logging function: log_e
Log level: error
Documentation ID: log_e-c9567c74dcbc4f1f

## Log pattern

```text
unsupported encryption method, chunk=%s, hls camera s=%d
```

## Parameters

- Argument 1 — url: Current origin URL.
- Argument 2 — socket: Origin connection socket descriptor.

## Log examples

Illustrative examples; these are not captured server logs.

```text
unsupported encryption method, chunk=https://origin.example/segment.ts, hls camera s=42
```

## Short description

HLS body has an unsupported encryption state.

## Trigger condition

Segment body processing encounters an encryption method other than unencrypted or AES-128.

## Explanation

Both chunked and fixed-length body paths request shutdown and return. The method number is not printed; playlist-level unsupported method text has its own diagnostic.

## Possible causes

- The active segment encryption state is outside the supported methods.

## What to check

1. Check preceding playlist encryption messages and selected segment encryption state.

## Recommended action

Use a supported unencrypted or AES-128 input.

## Related log messages

- [HLS playlist encryption method is unsupported](log_e-b934d2acca912062.md)
