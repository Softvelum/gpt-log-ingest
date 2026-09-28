# Cache cleanup encountered an unknown object

Product: Nimble Streamer
Component: Media data cache
Logging function: log_e
Log level: error
Documentation ID: log_e-b7a7e1cde04e3231

## Log pattern

```text
unknown cache_entity=%p
```

## Parameters

- Argument 1 — address: Memory address of the cache object, useful only for correlating nearby messages.

## Log examples

Illustrative examples; these are not captured server logs.

```text
unknown cache_entity=0x1000
```

## Short description

Cache cleanup encountered an unknown object.

## Trigger condition

An object in the expiration list is neither a cache item nor a fragment.

## Explanation

The cleanup pass counts and skips the object, then continues with the next entry.

## Possible causes

- The expiration list contains an unexpected object type.

## What to check

1. Inspect nearby cache list dumps and object addresses.

## Recommended action

Investigate the invalid cache list contents.
