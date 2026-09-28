# WHEP blocked request cannot await output

Product: Nimble Streamer
Component: WHEP playback helper
Logging function: log_e
Log level: error
Documentation ID: log_e-a2a80912bf0b05af

## Log pattern

```text
whep control socket failed to switch to output on new session request
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
whep control socket failed to switch to output on new session request
```

## Short description

WHEP blocked request cannot await output.

## Trigger condition

Sending a new WHEP request would block and enabling writable-event monitoring fails.

## Explanation

The new session is marked finished. The helper control socket and pending output are cleared, pending starts are marked finished, and unclassified accepted sockets are removed. Playback sockets already assigned to media workers are not directly closed by this cleanup.

## Possible causes

- The socket event monitor rejects the control-socket update.

## What to check

1. Check nearby control-connection and event-monitor errors.

## Recommended action

Restore the helper connection and retry playback.
