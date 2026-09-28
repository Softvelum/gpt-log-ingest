# Buffered NDI worker startup reported failure

Product: Nimble Streamer
Component: NDI media integration
Logging function: log_e
Log level: error
Documentation ID: log_e-9788ebc25c005def

## Log pattern

```text
failed to start ndi sender threads, ndi_name="%s"
```

## Parameters

- Argument 1 — NDI name: Requested or resolved NDI source name identifying the affected input or output.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to start ndi sender threads, ndi_name="HOST (Output)"
```

## Short description

Buffered NDI worker startup reported failure.

## Trigger condition

Creating either the video or audio sender thread returns false.

## Explanation

A shared exit request is set and sender initialization fails. The thread creation routine returns false when a thread object is already present; thrown thread-creation exceptions are not caught by this branch.

## Possible causes

- Creating either the video or audio sender thread returns false.

## What to check

1. Check whether sender-thread creation is being attempted more than once.

## Related log messages

- [Buffered NDI sender is starting its worker threads](log_i-f630d70b2c3afadd.md)
