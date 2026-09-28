# NVIDIA context destruction finished

Product: Nimble Streamer
Component: NVIDIA device contexts
Logging function: log_i
Log level: info
Documentation ID: log_i-f1ac72b5bcf3eb2e

## Log pattern

```text
destroyed nvenc ctx=%p, time=%ld(ms)
```

## Parameters

- Argument 1 — Context: CUDA context handle, pointer printed in hexadecimal.
- Argument 2 — Duration: Elapsed wall-clock duration, signed long milliseconds.

## Log examples

Illustrative examples; these are not captured server logs.

```text
destroyed nvenc ctx=0x12340000, time=12(ms)
```

## Short description

NVIDIA context destruction finished.

## Trigger condition

A nonshared context is released with caching enabled but is not returned to the reuse cache; the CUDA destruction call has returned.

## Explanation

The logged duration measures the destruction call. Its return status is not inspected, so the message records completion of the call rather than verified driver success.
