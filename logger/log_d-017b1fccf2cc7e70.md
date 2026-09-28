# RTMP playback waits for advertising content

Product: Nimble Streamer
Component: RTMP output and advertising
Logging function: log_d
Log level: debug
Documentation ID: log_d-017b1fccf2cc7e70

## Log pattern

```text
s=%d, waiting %d sec for content %s
```

## Parameters

- Argument 1 — Socket: Operating-system socket descriptor for the affected connection.
- Argument 2 — Wait: Configured content wait timeout in seconds.
- Argument 3 — Content URI: Configured URI of the selected advertising media item.

## Log examples

Illustrative examples; these are not captured server logs.

```text
s=42, waiting 5 sec for content https://media.example/ad.mp4
```

## Short description

RTMP playback waits for advertising content.

## Trigger condition

Selected content is not ready, is remote with a nonempty URI, and has a positive configured wait period.

## Explanation

The sender registers a load observer and timer, enters a waiting state, and disables output until load completion or timeout.

## Possible causes

- Selected content is not ready, is remote with a nonempty URI, and has a positive configured wait period.

## What to check

1. Check whether the content download completes within the displayed wait and the timeout error policy.

## Recommended action

Ensure the remote media can be loaded within the configured wait.
