# Cache object deletion postponed

Product: Nimble Streamer
Component: Media data cache
Logging function: log_v
Log level: verbose
Documentation ID: log_v-1ce5cf751ad0846d

## Log pattern

```text
postponed deletion of item p=%p key='%s'
```

## Parameters

- Argument 1 — address: Memory address of the cache object, useful only for correlating nearby messages.
- Argument 2 — key: Cache lookup key identifying the stored item.

## Log examples

Illustrative examples; these are not captured server logs.

```text
postponed deletion of item p=0x1000 key='movie'
```

## Short description

Cache object deletion postponed.

## Trigger condition

An item is removed from the lookup and reuse lists while deletion is explicitly deferred.

## Explanation

The object is retained in a separate set until its users release it. This allows a replacement key or explicit removal without immediately destroying an object still in use.
