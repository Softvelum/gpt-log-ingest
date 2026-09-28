# Windows cache file allocation failed

Product: Nimble Streamer
Component: Media data cache
Logging function: log_e
Log level: error
Documentation ID: log_e-96b60dc1a7cd3e9a

## Log pattern

```text
failed to create and map file='%s'
```

## Parameters

- Argument 1 — file: Filesystem path of the cache data or metadata file being handled.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to create and map file='/cache/movie'
```

## Short description

Windows cache file allocation failed.

## Trigger condition

The Windows file creation and mapping helper cannot allocate a normal cache item.

## Explanation

Cache allocation returns failure without attaching file storage to the item.

## Possible causes

- The operating system could not create or map the cache file.

## What to check

1. Check adjacent Windows file operation errors.

## Recommended action

Resolve the reported file or mapping problem.
