# RTMP AC-3 sequence start cannot be sent

Product: Nimble Streamer
Component: RTMP output and advertising
Logging function: log_e
Log level: error
Documentation ID: log_e-e6cf1b409a17ae84

## Log pattern

```text
failed to send AC3/E-AC3 sequence start to client: %d
```

## Parameters

- Argument 1 — Socket: Operating-system socket descriptor for the affected connection.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to send AC3/E-AC3 sequence start to client: 42
```

## Short description

RTMP AC-3 sequence start cannot be sent.

## Trigger condition

An outbound publisher or playback sender cannot send the generated enhanced RTMP sequence-start item for AC-3 or E-AC-3.

## Explanation

Output fails before the associated audio frame can proceed, and the caller closes the connection.

## Possible causes

- An outbound publisher or playback sender cannot send the generated enhanced RTMP sequence-start item for AC-3 or E-AC-3.

## What to check

1. Check transport errors for the affected connection and enhanced audio output.

## Recommended action

Resolve the send failure before restarting the session.
