# Scenario transcoder instance removed

Product: Nimble Streamer
Component: Scenario transcoder processes
Logging function: log_d
Log level: debug
Documentation ID: log_d-eeb953e4e38b65a7

## Log pattern

```text
removed scenario transcoder, id=%s
```

## Parameters

- Argument 1 — Scenario: Transcoding scenario identifier, text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
removed scenario transcoder, id=scenario1
```

## Short description

Scenario transcoder instance removed.

## Trigger condition

A scenario transcoder controller is being destroyed.

## Explanation

Any connection is removed and closed, its child is given 250 milliseconds to stop before termination is attempted, its output streams are unregistered, and its active x264 license sessions are released.
