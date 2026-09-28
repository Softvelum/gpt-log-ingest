# AAC encoder configuration retrieval failed

Product: Nimble Streamer
Component: FDK AAC audio encoding
Logging function: log_e
Log level: error
Documentation ID: log_e-8e0b3d219a2dd2d7

## Log pattern

```text
failed to get aac config, status=%d
```

## Parameters

- Argument 1 — Library status: Library-specific status code, integer; not an operating system error.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to get aac config, status=1
```

## Short description

AAC encoder configuration retrieval failed.

## Trigger condition

In an FDK-enabled build, obtaining AAC output configuration from the initialized encoder returns a non-success library status.

## Explanation

Encoder initialization fails and the triggering frame write returns failure.

## Possible causes

- FDK rejected encoder configuration retrieval for the selected audio settings.

## What to check

1. Check the reported FDK status and settings involved in encoder configuration retrieval.

## Recommended action

Use compatible AAC audio settings or resolve the reported library failure.
