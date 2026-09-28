# RTMP or RTSP publishing stops

Product: Nimble Streamer
Component: RTMP output and advertising
Logging function: log_v
Log level: verbose
Documentation ID: log_v-e73bca97a24149bb

## Log pattern

```text
stopped publishing %s
```

## Parameters

- Argument 1 — Publishing route: Previously recorded source application/stream, destination URL, and connected peer IP.

## Log examples

Illustrative examples; these are not captured server logs.

```text
stopped publishing stream='live/camera' to 'rtmp://destination.example:1935/live/camera' (192.0.2.20)
```

## Short description

RTMP or RTSP publishing stops.

## Trigger condition

A publisher with a recorded publishing route is destroyed.

## Explanation

The publisher statistics are marked inactive and the previous route is logged. The message alone does not identify why the connection ended.

## Possible causes

- A publisher with a recorded publishing route is destroyed.

## What to check

1. Check the preceding connection and route-management messages.

## Recommended action

No action is required for an intended stop; investigate earlier errors for unexpected stops.
