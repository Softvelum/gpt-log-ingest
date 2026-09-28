# Audio pipeline has no source

Product: Nimble Streamer
Component: Audio decoding
Logging function: log_e
Log level: error
Documentation ID: log_e-e4f0ffad2e7d0ffa

## Log pattern

```text
failed to get audio source
```

## Parameters

The decoder occurrence follows replacement of its source buffer. The encoder occurrence follows audio settings and synchronization-state updates.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to get audio source
```

## Short description

Audio pipeline has no source.

## Trigger condition

An audio decoder source update removes its live buffer, or an encoder audio update has neither a decoded-frame input nor a compressed live-stream buffer.

## Explanation

The update returns failure and the current worker processing cannot continue.

## Possible causes

- The configured audio source was removed or the new settings contain no usable input.

## What to check

1. Check audio source availability and the scenario update that changed the input.

## Recommended action

Restore a valid source or remove the dependent audio pipeline.
