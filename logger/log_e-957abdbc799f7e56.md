# Invalid route settings skipped

Product: Nimble Streamer
Component: WMSPanel synchronization
Logging function: log_e
Log level: error
Documentation ID: log_e-957abdbc799f7e56

## Log pattern

```text
failed to process route r='%s', skipping it
```

## Parameters

- Argument 1 — route: String representation obtained for the invalid route object.

## Log examples

The inspected wrapper delegates object-to-string conversion to the JSON library, so the exact route text is not established here.

## Short description

Invalid route settings skipped.

## Trigger condition

A route has a playlist caching interval of neither integer nor string type, another ordinary field that is not a string, a fallback-origins field that is not an array, or a fallback-origin element that is not an object.

## Explanation

That entire route is omitted while processing continues with later routes. Other valid routes may still be applied.

## Possible causes

- A route or fallback-origin member has an unexpected JSON type.

## What to check

1. Check the route field types, including playlist caching interval and fallback origins.

## Recommended action

Correct the invalid route JSON types.
