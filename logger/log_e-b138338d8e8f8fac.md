# Advertising redirect has no destination

Product: Nimble Streamer
Component: Advertising
Logging function: log_e
Log level: error
Documentation ID: log_e-b138338d8e8f8fac

## Log pattern

```text
empty redirect location, uri=%s
```

## Parameters

- Argument 1 — Content URI: Text giving the original advertising content location.

## Log examples

Illustrative examples; these are not captured server logs.

```text
empty redirect location, uri=https://media.example/ad.mp4
```

## Short description

Advertising redirect has no destination.

## Trigger condition

An advertising origin returns HTTP 301 or 302 with an empty redirect location.

## Explanation

The download session is marked finished.

## Possible causes

- An advertising origin returns HTTP 301 or 302 with an empty redirect location.

## What to check

1. Correct the origin response so the redirect includes a destination.

## Recommended action

Correct the origin response so the redirect includes a destination.
