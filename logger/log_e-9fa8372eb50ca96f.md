# AAC sample rate failed

Product: Nimble Streamer
Component: FDK AAC audio encoding
Logging function: log_e
Log level: error
Documentation ID: log_e-9fa8372eb50ca96f

## Log pattern

```text
failed to set sample_rate=%d, status=%d
```

## Parameters

- Argument 1 — Sample rate: Audio sample rate in samples per second, integer.
- Argument 2 — Library status: Library-specific status code, integer; not an operating system error.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to set sample_rate=48000, status=1
```

## Short description

AAC sample rate failed.

## Trigger condition

In an FDK-enabled build, setting the input sample rate returns a non-success library status.

## Explanation

Encoder initialization fails and the triggering frame write returns failure.

## Possible causes

- FDK rejected sample rate for the selected audio settings.

## What to check

1. Check the reported FDK status and settings involved in sample rate.

## Recommended action

Use compatible AAC audio settings or resolve the reported library failure.
