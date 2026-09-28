# Panel synchronization JSON prepared

Product: Nimble Streamer
Component: WMSPanel synchronization
Logging function: log_v
Log level: verbose
Documentation ID: log_v-1990cefa8481acc8

## Log pattern

```text
prepared sync packet len=%u so=%d body:
%s
```

## Parameters

- Argument 1 — body\_bytes: JSON body length in bytes, explicitly converted to an unsigned 32-bit integer.
- Argument 2 — settings\_only: 1 for a settings-only request, 0 for an ordinary sync.
- Argument 3 — json\_body: Complete prepared JSON body.

## Log examples

The body contains a large combined server report. No abbreviated example is supplied because an abbreviation would not represent the complete string emitted by this call.

## Short description

Panel synchronization JSON prepared.

## Trigger condition

The synchronization request body has been assembled before connecting to the selected panel.

## Explanation

The trace contains the complete JSON synchronization report and its included settings and statistics. The length covers JSON bytes, not HTTP headers. Settings-only requests skip ordinary statistics collection.
