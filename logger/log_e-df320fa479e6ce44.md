# Unhandled standard exception escaped a VOD request

Product: Nimble Streamer
Component: VOD transmuxing
Logging function: log_e
Log level: error
Documentation ID: log_e-df320fa479e6ce44

## Log pattern

```text
std::exception caught while processing request file='%s' chunk_name='%s' cache_item_key='%s' what()='%s'
```

## Parameters

- Argument 1 — Source media: Text filesystem path for local media, or source cache key for remote media.
- Argument 2 — Requested resource: Text name of the requested playlist, initialization segment, or media chunk.
- Argument 3 — Output cache key: Text cache key reserved for the requested generated output.
- Argument 4 — Exception detail: Text returned by the caught standard exception.

## Log examples

Illustrative examples; these are not captured server logs.

```text
std::exception caught while processing request file='/media/movie.mp4' chunk_name='n_0_0_0.ts' cache_item_key='/vod/movie.mp4/n_0_0_0.ts' what()='std::bad_alloc'
```

## Short description

Unhandled standard exception escaped a VOD request.

## Trigger condition

A standard exception escapes the request handler to the transmuxer worker loop.

## Explanation

The exception is logged and the loop continues with other work. This outer catch does not itself cancel or release the failed request output.

## Possible causes

- A standard exception escapes the request handler to the transmuxer worker loop.

## What to check

1. Use the detail and earlier messages for this output key to locate the failure; check whether a pending output was left waiting.

## Recommended action

Use the detail and earlier messages for this output key to locate the failure; check whether a pending output was left waiting.
