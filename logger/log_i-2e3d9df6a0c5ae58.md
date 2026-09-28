# Advertising content is ready

Product: Nimble Streamer
Component: Advertising
Logging function: log_i
Log level: info
Documentation ID: log_i-2e3d9df6a0c5ae58

## Log pattern

```text
loaded cached content id='%s' uri='%s'
```

## Parameters

- Argument 1 — Content ID: Text identifying the advertising content.
- Argument 2 — Content URI: Text giving the original advertising content location.

## Log examples

Illustrative examples; these are not captured server logs.

```text
loaded cached content id='ad-01' uri='https://media.example/ad.mp4'
```

## Short description

Advertising content is ready.

## Trigger condition

Content data is loaded and an audio or MP4 loader is available.

## Explanation

The content is marked ready. Remote content also notifies waiting advertising requests; local content needs no download notification.

## Possible causes

- Content data is loaded and an audio or MP4 loader is available.

## Related log messages

- [Advertising MP4 structure could not be loaded](log_e-ae902fa4b89ac257.md)
