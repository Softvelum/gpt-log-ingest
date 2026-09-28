# NVIDIA encoder CUDA context setup failed

Product: Nimble Streamer
Component: NVIDIA video encoding
Logging function: log_e
Log level: error
Documentation ID: log_e-77bce8b1bccbd751

## Log pattern

```text
failed to create cuda ctx
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to create cuda ctx
```

## Short description

NVIDIA encoder CUDA context setup failed.

## Trigger condition

Requesting the selected GPU encoder context from the context manager fails.

## Explanation

Encoder initialization stops before opening an encoding session.

## Possible causes

- GPU selection, context allocation or shared-context access fails.

## What to check

1. Check the preceding NVIDIA context-manager error for the specific operation.

## Recommended action

Correct that context failure before retrying encoding.

## Related log messages

- [Shared CUDA context detach failed](log_e-218a12322676b5e6.md)
- [Shared CUDA context binding failed](log_e-30bb8068b7404a23.md)
- [Shared CUDA context creation failed](log_e-7b9a9baf75f88d3d.md)
- [Shared CUDA context lock creation failed](log_e-82c870a24d9cbb83.md)
- [CUDA context creation failed](log_e-cb8c468650d1cb02.md)
- [Requested NVIDIA device index is unavailable](log_e-fcc0e449a77af017.md)
