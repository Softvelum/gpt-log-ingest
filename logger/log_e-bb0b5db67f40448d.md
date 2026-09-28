# WHEP incomplete request cannot await output

Product: Nimble Streamer
Component: WHEP playback helper
Logging function: log_e
Log level: error
Documentation ID: log_e-bb0b5db67f40448d

## Log pattern

```text
whep control socket failed to switch to output on partial send of new session request
```

## Parameters

This branch also includes a hard negative send result that was not handled as would-block.

## Log examples

Illustrative examples; these are not captured server logs.

```text
whep control socket failed to switch to output on partial send of new session request
```

## Short description

WHEP incomplete request cannot await output.

## Trigger condition

A new WHEP control request is not fully sent and enabling writable-event monitoring fails.

## Explanation

The new session is marked finished. The helper control socket and pending output are cleared, pending starts are marked finished, and unclassified accepted sockets are removed. Playback sockets already assigned to media workers are not directly closed by this cleanup.

## Possible causes

- The event monitor rejects the control-socket update after an incomplete send.

## What to check

1. Inspect nearby send and socket-monitoring failures.

## Recommended action

Restore the helper connection and retry.
