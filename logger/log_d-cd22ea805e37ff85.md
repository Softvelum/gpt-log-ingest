# First disk cache fragment allocated

Product: Nimble Streamer
Component: Media data cache
Logging function: log_d
Log level: debug
Documentation ID: log_d-cd22ea805e37ff85

## Log pattern

```text
allocated key='%s' fr=%llu sz=%llu disk=%llu
```

## Parameters

- Argument 1 — key: Cache lookup key identifying the stored item.
- Argument 2 — fragment: Zero-based fragment index.
- Argument 3 — allocated bytes: Size of the first mapped fragment in bytes.
- Argument 4 — disk bytes: Total cache capacity currently accounted for on disk, in bytes.

The fragment conversion expects a 64-bit integer but receives an ordinary integer zero; the size argument also uses a platform-dependent size type.

## Log examples

No reliable example: the fragment argument does not match its format conversion.

## Short description

First disk cache fragment allocated.

## Trigger condition

The first fragment file is created and mapped successfully.

## Explanation

The fragment enters the cache lists and its size is added to disk usage. This does not mark its downloaded content as available.
