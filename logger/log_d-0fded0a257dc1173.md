# Cached file reopened

Product: Nimble Streamer
Component: Media data cache
Logging function: log_d
Log level: debug
Documentation ID: log_d-0fded0a257dc1173

## Log pattern

```text
key='%s' opened '%s'
```

## Parameters

- Argument 1 — key: Cache lookup key identifying the stored item.
- Argument 2 — file: Filesystem path of the cache data or metadata file being handled.

## Log examples

Illustrative examples; these are not captured server logs.

```text
key='movie' opened '/var/cache/nimble/movie'
```

## Short description

Cached file reopened.

## Trigger condition

A non-Windows disk cache lookup successfully opens its stored file.

## Explanation

File-size checking and memory mapping still follow; later failures can make the lookup fail.
