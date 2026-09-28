# WebVTT queue reset after backward media jump

Product: Nimble Streamer
Component: WebVTT timing synchronization
Logging function: log_i
Log level: info
Documentation ID: log_i-bcee06194a0eac39

## Log pattern

```text
backward timestamp gap detected, reset webvtt sync queue for [%s/%s]
```

## Parameters

- Argument 1 — Application: Application name.
- Argument 2 — Stream: Stream name.

## Log examples

Illustrative examples; these are not captured server logs.

```text
backward timestamp gap detected, reset webvtt sync queue for [live/news]
```

## Short description

WebVTT queue reset after backward media jump.

## Trigger condition

An initialized audio or video timestamp moves backward by more than 30 seconds in its track timescale.

## Explanation

Queued messages are sent until a send fails, then the queue and pending subtitle state are cleared. The new timestamp begins a fresh synchronization history.

## Possible causes

- An initialized audio or video timestamp moves backward by more than 30 seconds in its track timescale.

## What to check

1. Check source timestamp continuity and whether the source restarted.
