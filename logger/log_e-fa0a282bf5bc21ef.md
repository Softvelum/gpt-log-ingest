# WebVTT subtitle cannot match buffered media

Product: Nimble Streamer
Component: WebVTT timing synchronization
Logging function: log_e
Log level: error
Documentation ID: log_e-fa0a282bf5bc21ef

## Log pattern

```text
failed to sync webvtt subtitle stream=[%s/%s]
```

## Parameters

- Argument 1 — Application: Application name.
- Argument 2 — Stream: Stream name.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to sync webvtt subtitle stream=[live/news]
```

## Short description

WebVTT subtitle cannot match buffered media.

## Trigger condition

A subtitle cannot match media within less than one second and is over one second behind the earliest buffered timestamp or over 30 seconds ahead of the latest.

## Explanation

The subtitle is classified as expired and discarded, either immediately or when a pending subtitle is retried.

## Possible causes

- A subtitle cannot match media within less than one second and is over one second behind the earliest buffered timestamp or over 30 seconds ahead of the latest.

## What to check

1. Compare subtitle timestamps with buffered audio/video timing and subtitle delivery delay.
