# Advertising download ended incomplete

Product: Nimble Streamer
Component: Advertising
Logging function: log_e
Log level: error
Documentation ID: log_e-011ac1acb7a04e57

## Log pattern

```text
failed to download content, uri=%s
```

## Parameters

- Argument 1 — Content URI: Text giving the original advertising content location.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to download content, uri=https://media.example/ad.mp4
```

## Short description

Advertising download ended incomplete.

## Trigger condition

An advertising download session is destroyed before its cached byte count reaches the allocated content size.

## Explanation

The cache item is canceled and the content returns to its initial state so it can be downloaded again.

## Possible causes

- An advertising download session is destroyed before its cached byte count reaches the allocated content size.

## What to check

1. Check earlier connection, HTTP response, and cache-allocation messages for this content.

## Recommended action

Check earlier connection, HTTP response, and cache-allocation messages for this content.
