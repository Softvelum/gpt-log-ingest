# Transcoder output stream registration failed

Product: Nimble Streamer
Component: Transcoder orchestration
Logging function: log_e
Log level: error
Documentation ID: log_e-d6d76b204fae93ea

## Log pattern

```text
register encoder stream %s %s failed
```

## Parameters

- Argument 1 — Application: Application name, text.
- Argument 2 — Stream: Stream name, text.

The five occurrences cover creating a filter output, adding an output to an existing filter, direct transcoding, and the explicit-stream and application-wide pass-through cases.

## Log examples

Illustrative examples; these are not captured server logs.

```text
register encoder stream live camera failed
```

## Short description

Transcoder output stream registration failed.

## Trigger condition

Registering a new transcoder output application and stream returns no stream registration.

## Explanation

The affected output is skipped; direct transcoder creation fails, while filter and pass-through synchronization continue with other outputs. The synchronization state is invalidated so a later synchronization can retry.

## Possible causes

- An existing stream with the same application and stream name prevents replacement; in a scenario encoder, any existing registration with that name conflicts.

## What to check

1. Check existing output registrations and duplicate output names.

## Recommended action

Assign unique output names or remove the conflicting stream registration.
