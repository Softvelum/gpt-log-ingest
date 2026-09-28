# HLS publishing thread started

Product: Nimble Streamer
Component: HTTP media delivery
Logging function: log_d
Log level: debug
Documentation ID: log_d-89eb09bee2bf5672

## Log pattern

```text
HlsPublisherThread started
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
HlsPublisherThread started
```

## Short description

HLS publishing thread started.

## Trigger condition

The HLS publishing thread enters its event loop.

## Explanation

It begins synchronizing configured publishers, checking inactivity, updating statistics, and processing notifications.

## Possible causes

- The HLS publishing thread enters its event loop.

## What to check

1. Check subsequent publisher start or request diagnostics.
