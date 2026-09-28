# Advertising playlist media parsing failed

Product: Nimble Streamer
Component: Advertising
Logging function: log_e
Log level: error
Documentation ID: log_e-b9b264a743cf14be

## Log pattern

```text
BaseMP4LoaderException caught while processing advertizer content, id='%s'
```

## Parameters

- Argument 1 — Content ID: Text identifying the advertising content.

## Log examples

Illustrative examples; these are not captured server logs.

```text
BaseMP4LoaderException caught while processing advertizer content, id='ad-01'
```

## Short description

Advertising playlist media parsing failed.

## Trigger condition

A media parser exception interrupts advertising playlist generation.

## Explanation

The playlist is marked failed and no ready playlist is published.

## Possible causes

- A media parser exception interrupts advertising playlist generation.

## What to check

1. Check the advertisement structure and selected audio and video tracks.

## Recommended action

Check the advertisement structure and selected audio and video tracks.
