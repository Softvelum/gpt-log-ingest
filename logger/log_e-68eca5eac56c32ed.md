# Cannot allocate HTTP media response buffer

Product: Nimble Streamer
Component: HLS and MPEG-TS HTTP input
Logging function: log_e
Log level: error
Documentation ID: log_e-68eca5eac56c32ed

## Log pattern

```text
cannot allocate buffer for s=%d url='%s'
```

## Parameters

- Argument 1 — socket: Origin connection socket descriptor.
- Argument 2 — url: Current origin URL.

## Log examples

Illustrative examples; these are not captured server logs.

```text
cannot allocate buffer for s=42 url='https://origin.example/live.ts'
```

## Short description

Cannot allocate HTTP media response buffer.

## Trigger condition

After the full request was sent and input events were selected, allocation of the response buffer at its maximum size fails.

## Explanation

The input requests shutdown. The requested maximum is 10 MiB for HLS input and 20 KiB for continuous MPEG-TS HTTP input.

## Possible causes

- The required incoming response-buffer allocation failed.

## What to check

1. Check available memory and preceding allocation diagnostics.

## Recommended action

Resolve the memory allocation failure.
