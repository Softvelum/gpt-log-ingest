# MPEG-TS HTTP response header parsing failed

Product: Nimble Streamer
Component: HLS and MPEG-TS HTTP input
Logging function: log_e
Log level: error
Documentation ID: log_e-c58cdc01703a1b1b

## Log pattern

```text
failed to parse buffer from s=%d
```

## Parameters

- Argument 1 — socket: Origin connection socket descriptor.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to parse buffer from s=42
```

## Short description

MPEG-TS HTTP response header parsing failed.

## Trigger condition

The HTTP response parser rejects the buffered input during header reception.

## Explanation

The input immediately enters the completed shutdown state and returns before checking response status or processing media.

## Possible causes

- The origin response is not accepted by the HTTP parser.

## What to check

1. Check the response header and whether the URL serves HTTP media.

## Recommended action

Correct the origin HTTP response.
