# Shared FFmpeg CUDA device created

Product: Nimble Streamer
Component: NVIDIA device contexts
Logging function: log_i
Log level: info
Documentation ID: log_i-71ea14daaa20f36a

## Log pattern

```text
created shared hw device ctx=%p, gpu=%s
```

## Parameters

- Argument 1 — Hardware context: FFmpeg hardware device reference address, pointer printed in hexadecimal.
- Argument 2 — GPU: CUDA device index passed to FFmpeg as text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
created shared hw device ctx=0x12340000, gpu=0
```

## Short description

Shared FFmpeg CUDA device created.

## Trigger condition

FFmpeg successfully creates a new CUDA hardware device context for the requested GPU.

## Explanation

The context is cached and a reference is returned to the caller.
