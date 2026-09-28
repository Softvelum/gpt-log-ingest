# HLS origin returned an unsuccessful HTTP status

Product: Nimble Streamer
Component: HLS and MPEG-TS HTTP input
Logging function: log_e
Log level: error
Documentation ID: log_e-8a7a87233d2ebed1

## Log pattern

```text
http error code=%d for url='%s' hls camera s=%d
```

## Parameters

- Argument 1 — http\_status: HTTP response status code.
- Argument 2 — url: Current origin URL.
- Argument 3 — socket: Origin connection socket descriptor.

## Log examples

Illustrative examples; these are not captured server logs.

```text
http error code=404 for url='https://origin.example/segment.ts' hls camera s=42
```

## Short description

HLS origin returned an unsuccessful HTTP status.

## Trigger condition

The completed HTTP header has a status other than 200, 301, or 302.

## Explanation

Without a redirect target, failures for segment, key, or initialization requests cause a new playlist request; failure while requesting a playlist shuts down the input. The message does not imply the entire server stops.

## Possible causes

- The origin returned a status not accepted for this HLS request.

## What to check

1. Check that the requested playlist, segment, initialization segment, or key is available.

## Recommended action

Correct the origin response or requested URL.
