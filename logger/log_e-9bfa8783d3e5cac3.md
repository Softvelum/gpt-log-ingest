# AAC audio bandwidth failed

Product: Nimble Streamer
Component: FDK AAC audio encoding
Logging function: log_e
Log level: error
Documentation ID: log_e-9bfa8783d3e5cac3

## Log pattern

```text
failed to set bandwidth=%d, status=%d
```

## Parameters

- Argument 1 — Bandwidth: Requested audio bandwidth in hertz, integer; zero selects the default.
- Argument 2 — Library status: Library-specific status code, integer; not an operating system error.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to set bandwidth=20000, status=1
```

## Short description

AAC audio bandwidth failed.

## Trigger condition

In an FDK-enabled build, setting the audio bandwidth returns a non-success library status.

## Explanation

Encoder initialization fails and the triggering frame write returns failure.

## Possible causes

- FDK rejected audio bandwidth for the selected audio settings.

## What to check

1. Check the reported FDK status and settings involved in audio bandwidth.

## Recommended action

Use compatible AAC audio settings or resolve the reported library failure.
