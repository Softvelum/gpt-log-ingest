# WHEP control output monitoring could not stop

Product: Nimble Streamer
Component: WHEP playback helper
Logging function: log_e
Log level: error
Documentation ID: log_e-d5729fd9b72daeed

## Log pattern

```text
whep control socket failed to disable output on partial send of new session request
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
whep control socket failed to disable output on partial send of new session request
```

## Short description

WHEP control output monitoring could not stop.

## Trigger condition

Pending output has drained, but switching the helper socket back to input/close monitoring fails.

## Explanation

The helper control socket and pending output are cleared, pending starts are marked finished, and unclassified accepted sockets are removed. Playback sockets already assigned to media workers are not directly closed by this cleanup.

## Possible causes

- The socket event monitor rejects the update after control output completes.

## What to check

1. Check for nearby helper disconnection or monitoring errors.

## Recommended action

Restore the helper connection; investigate repeated monitoring failures.
