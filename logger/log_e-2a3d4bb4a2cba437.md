# Live HTTP pull redirect URL is invalid

Product: Nimble Streamer
Component: Live input and publishing workers
Logging function: log_e
Log level: error
Documentation ID: log_e-2a3d4bb4a2cba437

## Log pattern

```text
invalid redirect location: %s
```

## Parameters

- Argument 1 — location: Redirect URL returned by the origin.

## Log examples

Illustrative examples; these are not captured server logs.

```text
invalid redirect location: invalid
```

## Short description

Live HTTP pull redirect URL is invalid.

## Trigger condition

The redirect lacks protocol and host parts or uses a scheme other than HTTP or HTTPS.

## Explanation

A probe connection is removed and periodic probing is restarted. A normal receiver is removed and its settings are scheduled for synchronization again.

## Possible causes

- The redirect lacks protocol and host parts or uses a scheme other than HTTP or HTTPS.

## What to check

1. Inspect the origin redirect target.

## Recommended action

Correct the redirect to a usable HTTP or HTTPS address.
