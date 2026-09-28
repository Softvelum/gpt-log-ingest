# Downloaded advertising content could not be loaded

Product: Nimble Streamer
Component: Advertising
Logging function: log_e
Log level: error
Documentation ID: log_e-e8b6bc814c511a1d

## Log pattern

```text
failed to load content, id=%s, uri=%s
```

## Parameters

- Argument 1 — Content ID: Text identifying the advertising content.
- Argument 2 — Content URI: Text giving the original advertising content location.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to load content, id=ad-01, uri=https://media.example/ad.mp4
```

## Short description

Downloaded advertising content could not be loaded.

## Trigger condition

A complete download is finalized but loading the resulting content fails.

## Explanation

The session continues cleanup; the content loader has not made the advertisement ready.

## Possible causes

- A complete download is finalized but loading the resulting content fails.

## What to check

1. Check that the downloaded object contains supported and valid advertising media.

## Recommended action

Check that the downloaded object contains supported and valid advertising media.
