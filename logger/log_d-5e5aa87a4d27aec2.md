# Active session restored

Product: Nimble Streamer
Component: Playback session tracking
Logging function: log_d
Log level: debug
Documentation ID: log_d-5e5aa87a4d27aec2

## Log pattern

```text
created active session=%ld
```

## Parameters

- Argument 1 — session: Numeric playback session identifier.

The format displays an unsigned-long identifier as signed long; the small positive example is within the common representable range.

## Log examples

Illustrative examples; these are not captured server logs.

```text
created active session=42
```

## Short description

Active session restored.

## Trigger condition

A valid saved active-session record is reconstructed by either loader.

## Explanation

The record is inserted in the active map, the identifier counter is advanced if needed, and the client-IP index is restored.
