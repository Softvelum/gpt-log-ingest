# Cache fragment removed

Product: Nimble Streamer
Component: Media data cache
Logging function: log_v
Log level: verbose
Documentation ID: log_v-b1cea4edf284d82a

## Log pattern

```text
removed key='%s' fr=%u sz=%u ref=%u file='%s' ram=%llu disk=%llu
```

## Parameters

- Argument 1 — key: Cache lookup key identifying the stored item.
- Argument 2 — fragment: Zero-based fragment index.
- Argument 3 — fragment bytes: Size deducted for the removed fragment.
- Argument 4 — references: Current number of users holding the item.
- Argument 5 — file: Filesystem path of the cache data or metadata file being handled.
- Argument 6 — RAM bytes: Total cache capacity currently allocated in memory, in bytes.
- Argument 7 — disk bytes: Total cache capacity currently accounted for on disk, in bytes.

## Log examples

Illustrative examples; these are not captured server logs.

```text
removed key='movie' fr=0 sz=1048576 ref=0 file='/var/cache/nimble/movie.fr/0' ram=0 disk=1048576
```

## Short description

Cache fragment removed.

## Trigger condition

A fragment is unmapped, its file removal is attempted, and its availability bit is cleared.

## Explanation

The parent item size and disk total are reduced. The fragment leaves cache ordering lists; if no availability bits remain, the parent item is removed too. Filesystem deletion errors are not checked at this message.
