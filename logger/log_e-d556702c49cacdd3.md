# Advertising chunk has no content loader

Product: Nimble Streamer
Component: Advertising
Logging function: log_e
Log level: error
Documentation ID: log_e-d556702c49cacdd3

## Log pattern

```text
failed to get content loader, content_id=%s
```

## Parameters

- Argument 1 — Content ID: Text identifying the advertising content.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to get content loader, content_id=ad-01
```

## Short description

Advertising chunk has no content loader.

## Trigger condition

An advertising MPEG-TS or audio chunk request has neither an MP4 loader nor an audio loader.

## Explanation

Generation returns and the pending cache item is canceled during request cleanup.

## Possible causes

- An advertising MPEG-TS or audio chunk request has neither an MP4 loader nor an audio loader.

## What to check

1. Ensure that the advertisement loaded successfully before requesting its chunks.

## Recommended action

Ensure that the advertisement loaded successfully before requesting its chunks.
