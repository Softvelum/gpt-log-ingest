# RTMP playback skips unavailable advertising content

Product: Nimble Streamer
Component: RTMP output and advertising
Logging function: log_e
Log level: error
Documentation ID: log_e-e8cb0f9d749b985c

## Log pattern

```text
s=%d, skip advertising content %s
```

## Parameters

- Argument 1 — Socket: Operating-system socket descriptor for the affected connection.
- Argument 2 — Content URI: Configured URI of the selected advertising media item.

## Log examples

Illustrative examples; these are not captured server logs.

```text
s=42, skip advertising content https://media.example/ad.mp4
```

## Short description

RTMP playback skips unavailable advertising content.

## Trigger condition

Content is not ready, cannot use the remote-content wait path, and its configured error action is skip.

## Explanation

The sender moves on to the next scheduled content item instead of stopping playback.

## Possible causes

- Content is not ready, cannot use the remote-content wait path, and its configured error action is skip.

## What to check

1. Check content readiness and the item skip-on-error policy.

## Recommended action

Restore the missing media if the advertisement must play.
