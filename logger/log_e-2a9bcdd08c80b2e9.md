# NVIDIA encoder shutdown flush failed

Product: Nimble Streamer
Component: NVIDIA video encoding
Logging function: log_e
Log level: error
Documentation ID: log_e-2a9bcdd08c80b2e9

## Log pattern

```text
failed to flush nvenc encoder
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to flush nvenc encoder
```

## Short description

NVIDIA encoder shutdown flush failed.

## Trigger condition

Sending end of stream to an existing encoder during destruction returns an error.

## Explanation

Destruction continues with input/output buffer, session and CUDA-context cleanup.

## Possible causes

- The NVIDIA driver rejects the requested cleanup operation.

## What to check

1. Check preceding encoding failures and NVIDIA device/driver state.

## Recommended action

If cleanup failures recur, restore the device/driver state before restarting encoding.
