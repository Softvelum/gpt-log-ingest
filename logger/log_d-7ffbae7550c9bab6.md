# WebSocket client frame type is unsupported

Product: Nimble Streamer
Component: SLDP playback and advertising
Logging function: log_d
Log level: debug
Documentation ID: log_d-7ffbae7550c9bab6

## Log pattern

```text
only non-chunked string/ping/close messages supported from client
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
only non-chunked string/ping/close messages supported from client
```

## Short description

WebSocket client frame type is unsupported.

## Trigger condition

The first frame byte is not a complete text, ping, or close frame with no reserved bits set.

## Explanation

Command decoding fails and the caller closes the connection. Fragmented commands and binary client commands are not accepted by this path.

## Possible causes

- The first frame byte is not a complete text, ping, or close frame with no reserved bits set.

## What to check

1. Inspect the opcode, FIN bit, and reserved bits in the client frame.

## Recommended action

Send complete text commands or supported ping/close frames.
