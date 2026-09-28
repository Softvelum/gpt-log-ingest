# Synchronization response has an unexpected JSON shape

Product: Nimble Streamer
Component: WMSPanel synchronization
Logging function: log_e
Log level: error
Documentation ID: log_e-d876c79d47ea89ba

## Log pattern

```text
unexpected response json='%s'
```

## Parameters

- Argument 1 — response\_body: Complete settings or synchronization JSON text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
unexpected response json='[]'
```

## Short description

Synchronization response has an unexpected JSON shape.

## Trigger condition

Parsed rules are not a JSON object, or a present SyncResponse member is not an object.

## Explanation

The parsed value is released and the optional parse-failure flag is set. The handler returns true so panel interaction can continue; it does not apply settings from this malformed structure.

## Possible causes

- The top-level or SyncResponse value has the wrong JSON type.

## What to check

1. Check that both levels use JSON objects.

## Recommended action

Correct the response structure.
