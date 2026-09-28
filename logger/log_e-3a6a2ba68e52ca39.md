# Cannot set SRT peer idle timeout

Product: Nimble Streamer
Component: SRT transport
Logging function: log_e
Log level: error
Documentation ID: log_e-3a6a2ba68e52ca39

## Log pattern

```text
failed to set peeridletimeo=%s s=%d
```

## Parameters

- Argument 1 — Value: Original configured peer idle timeout string, passed numerically without unit conversion.
- Argument 2 — Socket: SRT socket or group identifier.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to set peeridletimeo=5000 s=100
```

## Short description

Cannot set SRT peer idle timeout.

## Trigger condition

The SRT library rejects the configured peer idle timeout.

## Explanation

Applying socket settings stops with failure. The logged value is the original text converted to an integer before it is passed to SRT.

## Possible causes

- The SRT library rejects the configured peer idle timeout.

## What to check

1. Check the configured peer idle timeout and library compatibility.
