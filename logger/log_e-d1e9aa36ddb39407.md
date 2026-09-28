# Configuration value failed its specific validation

Product: Nimble Streamer
Component: Server configuration
Logging function: log_e
Log level: error
Documentation ID: log_e-d1e9aa36ddb39407

## Log pattern

```text
invalid parameter value: %s
```

## Parameters

- Argument 1 — Value: Original configuration value, text before numeric conversion; the applicable units depend on the named setting.

All occurrences print original text. Port values are integers; cache sizes are megabytes; session timeout is minutes. Other durations and limits use the units of their individual setting.

## Log examples

Illustrative examples; these are not captured server logs.

```text
invalid parameter value: 0
```

## Short description

Configuration value failed its specific validation.

## Trigger condition

A configuration branch rejects a parsed value or conversion: HTTP request or origin-playlist size text; management or panel port conversion; WHIP/WHEP/Zixi ports outside 0 through 65534 or invalid conversion state; worker counts at zero or below; negative RAM/disk cache sizes, session timeout, or inactivity timeout; nonnumeric RAM-cache threshold; VOD chunk duration outside 0 through 50; MPEG-TS inactivity interval outside 1 through 60; RTMP publisher inactivity interval outside 1 through 100; unrecognized session-load mode or chunk-split mode. The fixed RTMP size conversion has a defensive failure branch too.

## Explanation

Processing fails and configuration loading stops at the affected line. The pattern does not identify which validation failed.

## Possible causes

- The value does not meet the requirements of the setting being processed.

## What to check

1. Use the rejected configuration line to distinguish units and rules. Session-load modes are auto, entirely, partially; chunk-split modes are any, time, none. Size suffixes accepted by the size parser are uppercase K/KB, M/MB, and G/GB.

## Recommended action

Correct the value according to its setting and reload configuration.
