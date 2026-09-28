# Remote media fragment allocation failed

Product: Nimble Streamer
Component: Remote media cache requests
Logging function: log_e
Log level: error
Documentation ID: log_e-b7a041ea3bcf92e5

## Log pattern

```text
failed to allocate item='%s' fragment=%u
```

## Parameters

- Argument 1 — key: Cache key, also used as the remote request URL.
- Argument 2 — fragment: Zero-based fragment index.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to allocate item='https://media.example/movie.mp4' fragment=1
```

## Short description

Remote media fragment allocation failed.

## Trigger condition

The fragment has no storage and the cache cannot allocate it.

## Explanation

The fragment request fails before checking duplicate downloads or sending an origin request.

## Possible causes

- The data cache could not allocate the requested fragment.

## What to check

1. Check preceding cache capacity, fragment index, and file mapping errors.

## Recommended action

Correct the underlying cache allocation problem.
