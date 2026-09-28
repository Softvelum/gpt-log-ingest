# Scenario output stream registration failed

Product: Nimble Streamer
Component: Scenario transcoder processes
Logging function: log_e
Log level: error
Documentation ID: log_e-b18ec32d98df1d4c

## Log pattern

```text
failed to register stream %s for scenario=%s
```

## Parameters

- Argument 1 — Output stream: Output application and stream joined with a slash, text.
- Argument 2 — Scenario: Transcoding scenario identifier, text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to register stream live/output for scenario=scenario1
```

## Short description

Scenario output stream registration failed.

## Trigger condition

A scenario message references an output not already tracked by that scenario, but registering its application and stream returns no registration.

## Explanation

The message fails and connection processing fails; the scenario controller is removed and its resources are cleaned up.

## Possible causes

- An existing protected stream with the same application and stream name prevents registration.

## What to check

1. Check output-name collisions with other registered streams or scenarios.

## Recommended action

Give the scenario output a unique name or remove the conflicting registration.
