# Timed-out Icecast advertising stops playback

Product: Nimble Streamer
Component: HTTP client sessions
Logging function: log_e
Log level: error
Documentation ID: log_e-8f19bb61f475b1bf

## Log pattern

```text
timer expired, stop on advertising content error id=%s uri=%s
```

## Parameters

- Argument 1 — Content ID: Text identifying the advertising content.
- Argument 2 — Content URI: Text giving the original advertising content location.

## Log examples

Illustrative examples; these are not captured server logs.

```text
timer expired, stop on advertising content error id=ad-01 uri=https://media.example/ad.mp4
```

## Short description

Timed-out Icecast advertising stops playback.

## Trigger condition

An Icecast session waiting for advertising content reaches its timer and the configured error action is not skip.

## Explanation

HTTP/1 marks the client session finished. HTTP/2 or HTTP/3 resets the affected stream, with connection shutdown if reset handling fails.

## Possible causes

- An Icecast session waiting for advertising content reaches its timer and the configured error action is not skip.

## What to check

1. Investigate the advertisement readiness failure and decide whether the intended error action is to stop playback or skip unavailable advertising.

## Recommended action

Investigate the advertisement readiness failure and decide whether the intended error action is to stop playback or skip unavailable advertising.
