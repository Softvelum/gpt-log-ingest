# HLS item body download completed

Product: Nimble Streamer
Component: HLS and MPEG-TS HTTP input
Logging function: log_d
Log level: debug
Documentation ID: log_d-4bd89dc3ea0e353e

## Log pattern

```text
downloaded chunk='%s' hls camera s=%d
```

## Parameters

- Argument 1 — url: Current origin URL.
- Argument 2 — socket: Origin connection socket descriptor.

## Log examples

Illustrative examples; these are not captured server logs.

```text
downloaded chunk='https://origin.example/segment.ts' hls camera s=42
```

## Short description

HLS item body download completed.

## Trigger condition

The chunked HTTP parser reports the item complete, or the fixed-length remaining byte count reaches zero after body processing.

## Explanation

The current URL is logged and the input proceeds to request the next item. Despite the word chunk, these branches also handle encryption-key and initialization-segment downloads. It does not prove that every contained media frame was accepted.
