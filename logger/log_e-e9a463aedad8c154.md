# Standard exception interrupted media generation

Product: Nimble Streamer
Component: VOD transmuxing
Logging function: log_e
Log level: error
Documentation ID: log_e-e9a463aedad8c154

## Log pattern

```text
std::exception caught while processing file='%s' cache_item_key='%s' what()='%s'
```

## Parameters

- Argument 1 — Source media: Text filesystem path for local media, or source cache key for remote media.
- Argument 2 — Output cache key: Text cache key reserved for the requested generated output.
- Argument 3 — Exception detail: Text returned by the caught standard exception.

## Log examples

Illustrative examples; these are not captured server logs.

```text
std::exception caught while processing file='/media/movie.mp4' cache_item_key='/vod/movie.mp4/n_0_0_0.ts' what()='std::bad_alloc'
```

## Short description

Standard exception interrupted media generation.

## Trigger condition

A standard exception escapes the media-generation body and reaches its general exception handler.

## Explanation

The requested output cache item is canceled with a transmuxing-failure reason and released.

## Possible causes

- A standard exception escapes the media-generation body and reaches its general exception handler.

## What to check

1. Use the exception detail and preceding media or allocation diagnostics to identify the failed operation.

## Recommended action

Use the exception detail and preceding media or allocation diagnostics to identify the failed operation.
