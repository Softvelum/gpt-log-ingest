# Transport sender skips an item that exceeds its buffer

Product: Nimble Streamer
Component: MPEG-TS transport input and output
Logging function: log_e
Log level: error
Documentation ID: log_e-0feb93f32bad70f9

## Log pattern

```text
skip frame. buffer too small
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
skip frame. buffer too small
```

## Short description

Transport sender skips an item that exceeds its buffer.

## Trigger condition

A transport packet writer reports insufficient capacity while packaging the current video, audio, or metadata item.

## Explanation

The current item is skipped and constant-rate maintenance runs. This branch does not itself force sender shutdown.

## Possible causes

- A transport packet writer reports insufficient capacity while packaging the current video, audio, or metadata item.

## What to check

1. Check media item size, codec-header overhead, and output capacity.

## Recommended action

Resolve excessive item size or the output-buffer sizing problem.
