# Cannot store HLS HTTP request

Product: Nimble Streamer
Component: HLS and MPEG-TS HTTP input
Logging function: log_e
Log level: error
Documentation ID: log_e-27a3d8fd33aaec53

## Log pattern

```text
cannot allocate buffer for hls request s=%d url='%s'
```

## Parameters

- Argument 1 — socket: Origin connection socket descriptor.
- Argument 2 — url: Current origin URL.

## Log examples

Illustrative examples; these are not captured server logs.

```text
cannot allocate buffer for hls request s=-1 url='https://origin.example/live.m3u8'
```

## Short description

Cannot store HLS HTTP request.

## Trigger condition

Writing the prepared request into the outgoing buffer fails.

## Explanation

Request preparation requests shutdown and returns false. The request buffer has a 16 KiB maximum; this may happen before a new socket has been connected, so its descriptor may be -1.

## Possible causes

- The request did not fit or its buffer could not be allocated.

## What to check

1. Check generated URL/header size and preceding memory diagnostics.

## Recommended action

Reduce oversized request data or resolve the reported allocation problem.
