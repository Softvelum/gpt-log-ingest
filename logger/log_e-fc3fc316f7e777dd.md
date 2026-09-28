# Downloaded fragment has no destination buffer

Product: Nimble Streamer
Component: Remote media cache requests
Logging function: log_e
Log level: error
Documentation ID: log_e-fc3fc316f7e777dd

## Log pattern

```text
failed to get item='%s' fragment=%u
```

## Parameters

- Argument 1 — key: Cache key, also used as the remote request URL.
- Argument 2 — fragment: Zero-based fragment index.

## Log examples

No ordinary input is established: the request path keeps an allocated fragment alive until this callback.

## Short description

Downloaded fragment has no destination buffer.

## Trigger condition

The requested fragment cannot supply a data pointer when its completed response is copied.

## Explanation

The copy is skipped, but the code still releases the fragment and marks its range available afterward. The message therefore reports an internal inconsistency, not a clean download rejection.

## Possible causes

- The completed request lacks the expected allocated fragment buffer.

## What to check

1. Check prior fragment allocation and cleanup for the key.

## Recommended action

Investigate the cache ownership inconsistency before relying on that fragment.
