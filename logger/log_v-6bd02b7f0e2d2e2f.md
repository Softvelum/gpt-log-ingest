# Loaded cache ordering is being rebuilt

Product: Nimble Streamer
Component: Media data cache
Logging function: log_v
Log level: verbose
Documentation ID: log_v-6bd02b7f0e2d2e2f

## Log pattern

```text
sorting cache items lists after loading
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
sorting cache items lists after loading
```

## Short description

Loaded cache ordering is being rebuilt.

## Trigger condition

All cache directories have been scanned and loaded entries need ordering.

## Explanation

The cache reconstructs expiration and reuse lists, keeping live and VOD categories separate before reporting loading completion.
