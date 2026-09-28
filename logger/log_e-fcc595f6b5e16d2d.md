# NVIDIA decoder CUDA context setup failed

Product: Nimble Streamer
Component: NVIDIA video decoding
Logging function: log_e
Log level: error
Documentation ID: log_e-fcc595f6b5e16d2d

## Log pattern

```text
failed to create nvdec cuda ctx for [%s]
```

## Parameters

- Argument 1 — Stream: Input stream name, text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to create nvdec cuda ctx for [live/camera]
```

## Short description

NVIDIA decoder CUDA context setup failed.

## Trigger condition

The decoder has no CUDA context, and requesting one from the context manager fails.

## Explanation

The current input packet returns failure before parser creation.

## Possible causes

- GPU selection, context allocation, sharing, locking or activation fails.

## What to check

1. Check the preceding context-manager diagnostic for the GPU and precise operation.

## Recommended action

Correct the context setup error before retrying decoding.

## Related log messages

- [Shared CUDA context detach failed](log_e-218a12322676b5e6.md)
- [Shared CUDA context binding failed](log_e-30bb8068b7404a23.md)
- [CUDA decoder context activation failed](log_e-7767dd3aab7b7233.md)
- [Shared CUDA context creation failed](log_e-7b9a9baf75f88d3d.md)
- [Shared CUDA context lock creation failed](log_e-82c870a24d9cbb83.md)
- [CUDA context creation failed](log_e-cb8c468650d1cb02.md)
- [Requested NVIDIA device index is unavailable](log_e-fcc0e449a77af017.md)
