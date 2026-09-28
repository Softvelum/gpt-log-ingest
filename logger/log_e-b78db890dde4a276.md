# RTMP advertising wait expires and stops playback

Product: Nimble Streamer
Component: RTMP output and advertising
Logging function: log_e
Log level: error
Documentation ID: log_e-b78db890dde4a276

## Log pattern

```text
s=%d, timer expired, stop on advertising content error %s
```

## Parameters

- Argument 1 — Socket: Operating-system socket descriptor for the affected connection.
- Argument 2 — Content URI: Configured URI of the selected advertising media item.

## Log examples

Illustrative examples; these are not captured server logs.

```text
s=42, timer expired, stop on advertising content error https://media.example/ad.mp4
```

## Short description

RTMP advertising wait expires and stops playback.

## Trigger condition

The matching content-wait timer expires and the item is not configured to skip errors.

## Explanation

The load observer and timer are removed, and the playback connection is shut down.

## Possible causes

- The matching content-wait timer expires and the item is not configured to skip errors.

## What to check

1. Check the media download, wait timeout, and error action for this item.

## Recommended action

Restore content availability or correct the intended timeout policy.
