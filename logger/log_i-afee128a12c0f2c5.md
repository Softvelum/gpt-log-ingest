# Untracked NVIDIA context will be destroyed

Product: Nimble Streamer
Component: NVIDIA device contexts
Logging function: log_i
Log level: info
Documentation ID: log_i-afee128a12c0f2c5

## Log pattern

```text
unknown nvenc ctx=%p will be destroyed
```

## Parameters

- Argument 1 — Context: CUDA context handle, pointer printed in hexadecimal.

## Log examples

Illustrative examples; these are not captured server logs.

```text
unknown nvenc ctx=0x12340000 will be destroyed
```

## Short description

Untracked NVIDIA context will be destroyed.

## Trigger condition

Caching and reuse are enabled, but the context being released is absent from the in-use map.

## Explanation

The context cannot be returned to the free cache and will be passed to CUDA destruction.

## Possible causes

- The released handle is not tracked as a checked-out reusable context.

## What to check

1. Correlate context creation, checkout and earlier return messages for this handle.

## Recommended action

No action is needed if destruction is expected; investigate tracking if reuse was expected.

## Related log messages

- [NVIDIA context destruction finished](log_i-f1ac72b5bcf3eb2e.md)
