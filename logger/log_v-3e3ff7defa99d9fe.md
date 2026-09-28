# Filter stream wildcard resolved

Product: Nimble Streamer
Component: Video filtering
Logging function: log_v
Log level: verbose
Documentation ID: log_v-3e3ff7defa99d9fe

## Log pattern

```text
resolved wildcard filter_expression="%s" for stream='%s' to "%s"
```

## Parameters

- Argument 1 — Expression: Original video filter expression, text.
- Argument 2 — Main input: Main video stream name, text.
- Argument 3 — Resolved expression: Video filter expression after replacement, text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
resolved wildcard filter_expression="drawtext=textfile='/tmp/{STREAM}.txt'" for stream='live/camera' to "drawtext=textfile='/tmp/camera.txt'"
```

## Short description

Filter stream wildcard resolved.

## Trigger condition

A quoted textfile option contains {STREAM} and the main name splits into exactly two parts.

## Explanation

Matching placeholders are replaced by the stream part and the resulting expression is used.
