# Seekable local media response assembled

Product: Nimble Streamer
Component: HTTP client sessions
Logging function: log_d
Log level: debug
Documentation ID: log_d-1ea4a582c30e7d2c

## Log pattern

```text
resp out_chain.size=%u elements, header:
%s
```

## Parameters

- Argument 1 — Output-chain elements: Native-size unsigned count of memory/file output buffers, incorrectly printed with %u.
- Argument 2 — Response header: Text of the generated HTTP response headers.

The output-chain length is a native-size unsigned count passed to %u, which is unreliable where size_t is wider than unsigned int.

## Log examples

The output-chain length is a native-size unsigned count passed to %u, which is unreliable where size_t is wider than unsigned int.

## Short description

Seekable local media response assembled.

## Trigger condition

A local MP4 or FLV request with a start parameter is handled by its media module.

## Explanation

The generated response headers and output-chain count are logged before the chain is transferred to client sending.

## Possible causes

- A local MP4 or FLV request with a start parameter is handled by its media module.

## What to check

1. Not applicable; this is normal diagnostic progress. Use the socket identifier to follow the same request if troubleshooting.

## Recommended action

Not applicable; this is normal diagnostic progress. Use the socket identifier to follow the same request if troubleshooting.
