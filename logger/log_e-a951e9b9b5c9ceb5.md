# Cache space recovery encountered an unknown object

Product: Nimble Streamer
Component: Media data cache
Logging function: log_e
Log level: error
Documentation ID: log_e-a951e9b9b5c9ceb5

## Log pattern

```text
unknown entity to remove p=%p
```

## Parameters

- Argument 1 — address: Memory address of the cache object, useful only for correlating nearby messages.

## Log examples

No normal triggering input is established by the object selection logic.

## Short description

Cache space recovery encountered an unknown object.

## Trigger condition

An object selected for space recovery is neither an item nor a fragment at removal time.

## Explanation

The loop logs the unexpected object and continues removing selected entries. Its selection logic normally only adds items and fragments, so ordinary input does not establish a reachable example.

## Possible causes

- The removal list contains an object that fails both expected type checks.

## What to check

1. Compare cache list diagnostics around the space-recovery attempt.

## Recommended action

Investigate invalid cache object state.
