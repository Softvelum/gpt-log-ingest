# Video filter worker started

Product: Nimble Streamer
Component: Video filtering
Logging function: log_d
Log level: debug
Documentation ID: log_d-8faff870511dfd42

## Log pattern

```text
VideoFilterThread(%s) started
```

## Parameters

- Argument 1 — Filter: Configured filter identifier, text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
VideoFilterThread(overlay1) started
```

## Short description

Video filter worker started.

## Trigger condition

The video filter worker starts before constructing its graph wrapper and input links.

## Explanation

Filter creation and frame processing follow; this does not confirm the graph can accept frames.
