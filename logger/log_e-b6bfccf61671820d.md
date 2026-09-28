# WHEP control welcome send failed

Product: Nimble Streamer
Component: WHEP playback helper
Logging function: log_e
Log level: error
Documentation ID: log_e-b6bfccf61671820d

## Log pattern

```text
whep control socket failed to send Control Welcome message request
```

## Parameters

The three occurrences cover zero-byte send, failed monitoring after would-block, and failed monitoring after another incomplete send. A hard negative send can enter the final branch.

## Log examples

Illustrative examples; these are not captured server logs.

```text
whep control socket failed to send Control Welcome message request
```

## Short description

WHEP control welcome send failed.

## Trigger condition

Sending the control welcome returns zero, or an incomplete/would-block send cannot enable writable-event monitoring.

## Explanation

The welcome operation returns failure; its caller logs a further welcome failure and clears the helper control connection and pending starts.

## Possible causes

- The control connection cannot send, or its pending output cannot be monitored.

## What to check

1. Check helper connection availability and preceding socket event errors.

## Recommended action

Restore the control connection before starting playback sessions.

## Related log messages

- [WHEP control welcome could not be sent](log_e-b60c2feea6803f9a.md)
