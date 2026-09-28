# WebSocket command uses unsupported 64-bit length framing

Product: Nimble Streamer
Component: SLDP playback and advertising
Logging function: log_d
Log level: debug
Documentation ID: log_d-ab3e586354f30eac

## Log pattern

```text
we don't support huge messages
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
we don't support huge messages
```

## Short description

WebSocket command uses unsupported 64-bit length framing.

## Trigger condition

The client frame uses payload-length marker 127.

## Explanation

The frame is rejected before reading its 64-bit length. This is a framing-form check, including any shorter payload incorrectly encoded with that marker.

## Possible causes

- The client frame uses payload-length marker 127.

## What to check

1. Check the frame payload-length marker.

## Recommended action

Use the supported short or 16-bit length form for commands.
