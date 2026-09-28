# Canceled cache fetch falls back to origin

Product: Nimble Streamer
Component: HTTP client sessions
Logging function: log_d
Log level: debug
Documentation ID: log_d-5a2500fdfeb5221d

## Log pattern

```text
session=%p fallback to pass-through mode
```

## Parameters

- Argument 1 — Session address: Pointer identifying this client session within the process.

## Log examples

Illustrative examples; these are not captured server logs.

```text
session=0x1234 fallback to pass-through mode
```

## Short description

Canceled cache fetch falls back to origin.

## Trigger condition

An item is marked for removal while the client waits for cached headers or a cached playlist.

## Explanation

The cache observer/item is released and a direct origin connection is started so the request can proceed without that cache item.

## Possible causes

- An item is marked for removal while the client waits for cached headers or a cached playlist.

## What to check

1. Check preceding cache-allocation or origin-download errors and the outcome of the direct origin connection.

## Recommended action

Check preceding cache-allocation or origin-download errors and the outcome of the direct origin connection.
