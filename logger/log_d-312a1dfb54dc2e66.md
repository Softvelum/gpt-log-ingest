# Advertising subtitle resource is disabled

Product: Nimble Streamer
Component: VOD transmuxing
Logging function: log_d
Log level: debug
Documentation ID: log_d-312a1dfb54dc2e66

## Log pattern

```text
invalid ads subtitles chunk request [%s] with disabled Addenda
```

## Parameters

- Argument 1 — Advertising subtitle resource: Text requested resource with an advertising subtitle suffix.

## Log examples

Illustrative examples; these are not captured server logs.

```text
invalid ads subtitles chunk request [n_0_fmp4.webvtt] with disabled Addenda
```

## Short description

Advertising subtitle resource is disabled.

## Trigger condition

A WebVTT request identifies an advertising MPEG-TS or fragmented-MP4 subtitle chunk while the advertising service URL is empty.

## Explanation

The requested output cache item is canceled and released; this attempt produces no usable output.

## Possible causes

- A WebVTT request identifies an advertising MPEG-TS or fragmented-MP4 subtitle chunk while the advertising service URL is empty.

## What to check

1. Check advertising enablement and whether the player should still be using an advertising subtitle playlist. The branch tests the advertising URL, despite the fixed Addenda wording.

## Recommended action

Check advertising enablement and whether the player should still be using an advertising subtitle playlist. The branch tests the advertising URL, despite the fixed Addenda wording.
