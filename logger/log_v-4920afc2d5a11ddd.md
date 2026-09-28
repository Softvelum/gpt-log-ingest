# HLS publishing instance stopped

Product: Nimble Streamer
Component: HTTP media delivery
Logging function: log_v
Log level: verbose
Documentation ID: log_v-4920afc2d5a11ddd

## Log pattern

```text
stop publishing %s
```

## Parameters

- Argument 1 — Description: Publishing summary containing source application/stream and destination URL.

## Log examples

Illustrative examples; these are not captured server logs.

```text
stop publishing stream='live/news' to https://upload.example.net/live/
```

## Short description

HLS publishing instance stopped.

## Trigger condition

An HLS publisher is destroyed after it has recorded publishing information.

## Explanation

Its statistics are marked inactive and its chunk observer is removed. The reason can be configuration removal, shutdown, inactivity, or request failure.

## Possible causes

- An HLS publisher is destroyed after it has recorded publishing information.

## What to check

1. Check nearby publisher errors or configuration changes.
