# Advertising content is unavailable for a chunk

Product: Nimble Streamer
Component: HTTP client sessions
Logging function: log_e
Log level: error
Documentation ID: log_e-81dd1d21ae29cf3f

## Log pattern

```text
failed to get content=%s
```

## Parameters

- Argument 1 — Content ID: Text identifying the advertising content.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to get content=ad-01
```

## Short description

Advertising content is unavailable for a chunk.

## Trigger condition

The advertising content lookup returns no item or an item that is not ready.

## Explanation

HTTP 500 is prepared instead of scheduling or sending the requested advertising chunk.

## Possible causes

- The advertising content lookup returns no item or an item that is not ready.

## What to check

1. Check the content identifier and preceding download or preparation failures; a chunk request requires ready advertising content.

## Recommended action

Check the content identifier and preceding download or preparation failures; a chunk request requires ready advertising content.
