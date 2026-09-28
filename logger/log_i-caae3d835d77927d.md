# Buffered NDI audio sender stopped

Product: Nimble Streamer
Component: NDI media integration
Logging function: log_i
Log level: info
Documentation ID: log_i-caae3d835d77927d

## Log pattern

```text
ndi audio sender stoped, ndi_name="%s"
```

## Parameters

- Argument 1 — NDI name: Requested or resolved NDI source name identifying the affected input or output.

## Log examples

Illustrative examples; these are not captured server logs.

```text
ndi audio sender stoped, ndi_name="HOST (Output)"
```

## Short description

Buffered NDI audio sender stopped.

## Trigger condition

The dedicated audio worker leaves its send loop.

## Explanation

For audio output, this follows an exit request. The message follows completion of the send loop.

## Possible causes

- An exit request.

## What to check

1. Check the preceding messages if the worker stopped unexpectedly.

## Related log messages

- [Buffered NDI audio sender started](log_i-593de6c3e3aebdd4.md)
