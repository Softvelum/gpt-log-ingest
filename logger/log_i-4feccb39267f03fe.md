# Transmuxing resumed at bus-signal recovery

Product: Nimble Streamer
Component: VOD transmuxing
Logging function: log_i
Log level: info
Documentation ID: log_i-4feccb39267f03fe

## Log pattern

```text
returned after SIGBUS is received
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
returned after SIGBUS is received
```

## Short description

Transmuxing resumed at bus-signal recovery.

## Trigger condition

Control returns to the saved recovery point after the non-Windows bus-signal handler interrupts mapped-media processing.

## Explanation

The request is abandoned. Audio handling closes its local mapping and cancels output; general handling removes a local mapped file from its open-file cache and cancels output that is still pending or downloading.

## Possible causes

- Control returns to the saved recovery point after the non-Windows bus-signal handler interrupts mapped-media processing.

## What to check

1. Inspect the preceding bus-signal details and changes to the source file; replace media atomically rather than truncating a file currently being mapped.

## Recommended action

Inspect the preceding bus-signal details and changes to the source file; replace media atomically rather than truncating a file currently being mapped.

## Related log messages

- [Transmuxer caught a bus signal](log_i-651ef91257c22c8e.md)
- [Bus-signal recovery checks generated output state](log_i-66f18f07a99daf1f.md)
- [Bus-signal recovery records request objects](log_i-97495d440ea0f7fd.md)
