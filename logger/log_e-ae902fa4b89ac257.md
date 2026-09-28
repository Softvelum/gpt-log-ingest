# Advertising MP4 structure could not be loaded

Product: Nimble Streamer
Component: Advertising
Logging function: log_e
Log level: error
Documentation ID: log_e-ae902fa4b89ac257

## Log pattern

```text
BaseMP4LoaderException caught while processing advertizer content, id=%s, uri=%s
```

## Parameters

- Argument 1 — Content ID: Text identifying the advertising content.
- Argument 2 — Content URI: Text giving the original advertising content location.

## Log examples

Illustrative examples; these are not captured server logs.

```text
BaseMP4LoaderException caught while processing advertizer content, id=ad-01, uri=https://media.example/ad.mp4
```

## Short description

Advertising MP4 structure could not be loaded.

## Trigger condition

Loading the advertising MP4 structure raises a media parser exception.

## Explanation

The content returns to its initial state and loading returns failure.

## Possible causes

- Loading the advertising MP4 structure raises a media parser exception.

## What to check

1. Check the MP4 structure of the indicated advertisement.

## Recommended action

Check the MP4 structure of the indicated advertisement.

## Related log messages

- [Advertising content is ready](log_i-2e3d9df6a0c5ae58.md)
