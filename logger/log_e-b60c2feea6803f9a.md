# WHEP control welcome could not be sent

Product: Nimble Streamer
Component: WHEP playback helper
Logging function: log_e
Log level: error
Documentation ID: log_e-b60c2feea6803f9a

## Log pattern

```text
whep control socket failed to send control welcome message request from ProcessEvent
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
whep control socket failed to send control welcome message request from ProcessEvent
```

## Short description

WHEP control welcome could not be sent.

## Trigger condition

A newly identified WHEP control connection is promoted, but preparing or sending its control welcome returns failure.

## Explanation

The helper control socket and pending output are cleared, pending starts are marked finished, and unclassified accepted sockets are removed. Playback sockets already assigned to media workers are not directly closed by this cleanup.

## Possible causes

- The welcome cannot be buffered or sent, or pending output cannot be registered with the event monitor.

## What to check

1. Check preceding control-welcome send errors and helper connection availability.

## Recommended action

Restore the helper control connection before retrying playback starts.

## Related log messages

- [WHEP control welcome send failed](log_e-b6bfccf61671820d.md)
