# Background origin read would block

Product: Nimble Streamer
Component: Background origin download
Logging function: log_e
Log level: error
Documentation ID: log_e-ff291e4beef4a0a7

## Log pattern

```text
Read IsWouldBlockError=%d on s=%d (origin) key='%s'
```

## Parameters

- Argument 1 — error: Operating system would-block error.
- Argument 2 — socket: Origin socket descriptor.
- Argument 3 — key: Cache key, also used as the remote request URL.

## Log examples

Illustrative examples; these are not captured server logs.

```text
Read IsWouldBlockError=11 on s=12 (origin) key='https://media.example/movie.mp4'
```

## Short description

Background origin read would block.

## Trigger condition

A plain origin body read fails with a temporary would-block error.

## Explanation

This downloader treats that unexpected temporary failure as completion of the session rather than scheduling another plain read. Cleanup determines whether the cache item is complete or canceled.

## Possible causes

- No body data was available when the plain download read ran.

## What to check

1. Compare socket readiness and downloaded byte totals.

## Recommended action

Investigate unexpected read readiness if downloads stop before completion.
