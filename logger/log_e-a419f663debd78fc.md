# No management listening interface opened

Product: Nimble Streamer
Component: HTTP management API
Logging function: log_e
Log level: error
Documentation ID: log_e-a419f663debd78fc

## Log pattern

```text
was not able to connect any interface specified.
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
was not able to connect any interface specified.
```

## Short description

No management listening interface opened.

## Trigger condition

The configured management interface text is nonempty but listener creation leaves no listening sockets.

## Explanation

Despite the word connect, this concerns local listening sockets. The management thread then logs that it stopped and returns. No error is printed by this branch when the interface text is empty.

## Possible causes

- No configured plain or TLS management listener was successfully created.

## What to check

1. Check the configured management addresses and positive HTTP or HTTPS port values and preceding socket errors.

## Recommended action

Correct the management listening settings or socket failure indicated by earlier messages.

## Related log messages

- [Management listener thread stopped](log_d-513c7e9841541526.md)
