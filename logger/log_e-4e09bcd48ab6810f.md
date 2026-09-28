# NVIDIA input-buffer destruction failed

Product: Nimble Streamer
Component: NVIDIA video encoding
Logging function: log_e
Log level: error
Documentation ID: log_e-4e09bcd48ab6810f

## Log pattern

```text
failed to destroy nvenc input buffer
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to destroy nvenc input buffer
```

## Short description

NVIDIA input-buffer destruction failed.

## Trigger condition

Destroying an initialized encoder input buffer during cleanup returns an error.

## Explanation

Cleanup continues with the paired bitstream buffer and remaining encoder resources.

## Possible causes

- The NVIDIA driver rejects the requested cleanup operation.

## What to check

1. Check preceding encoding failures and NVIDIA device/driver state.

## Recommended action

If cleanup failures recur, restore the device/driver state before restarting encoding.
