# Buffered NDI audio sender started

Product: Nimble Streamer
Component: NDI media integration
Logging function: log_i
Log level: info
Documentation ID: log_i-593de6c3e3aebdd4

## Log pattern

```text
ndi audio sender started, ndi_name="%s"
```

## Parameters

- Argument 1 — NDI name: Requested or resolved NDI source name identifying the affected input or output.

## Log examples

Illustrative examples; these are not captured server logs.

```text
ndi audio sender started, ndi_name="HOST (Output)"
```

## Short description

Buffered NDI audio sender started.

## Trigger condition

The dedicated buffered audio sender enters its worker routine.

## Explanation

It begins reading and pacing audio frames until exit is requested.

## Possible causes

- Normal startup of the buffered NDI audio worker.

## Related log messages

- [Buffered NDI audio sender stopped](log_i-caae3d835d77927d.md)
