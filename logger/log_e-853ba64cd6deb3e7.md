# Local device stream manager could not start

Product: Nimble Streamer
Component: Transcoder orchestration
Logging function: log_e
Log level: error
Documentation ID: log_e-853ba64cd6deb3e7

## Log pattern

```text
failed to create local network stream manager thread
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to create local network stream manager thread
```

## Short description

Local device stream manager could not start.

## Trigger condition

Creating the local network/device stream manager thread fails at transcoder startup.

## Explanation

The manager instance is discarded; the main transcoder worker continues without that helper.

## Possible causes

- The operating system cannot create the requested worker thread.

## What to check

1. Check thread and process resource availability during transcoder startup.

## Recommended action

Restore thread resources and restart transcoder operation to create the helper.
