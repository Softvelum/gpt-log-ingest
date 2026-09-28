# Automatic video packet changed media type

Product: Nimble Streamer
Component: MPEG-TS live stream selection
Logging function: log_d
Log level: debug
Documentation ID: log_d-5cfbf3bb10126df2

## Log pattern

```text
updateAvailable video type changed for actual_video_pid=%d from %d to %d
```

## Parameters

- Argument 1 — packet\_id: MPEG-TS packet identifier printed as a decimal integer.
- Argument 2 — previous\_type: Previously selected stream-type code in decimal.
- Argument 3 — new\_type: New advertised stream-type code in decimal.

## Log examples

Illustrative examples; these are not captured server logs.

```text
updateAvailable video type changed for actual_video_pid=256 from 27 to 36
```

## Short description

Automatic video packet changed media type.

## Trigger condition

The active automatically selected video identifier still equals the current default, but its advertised stream type changed.

## Explanation

Video codec initialization, timestamps, parameter sets, and frame assembly buffers are reset. The new type is retained and the identifier remains requested.
