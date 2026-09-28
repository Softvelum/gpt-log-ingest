# OMT receiver creation failed

Product: Nimble Streamer
Component: OMT media integration
Logging function: log_e
Log level: error
Documentation ID: log_e-f45c3e39c9cb858d

## Log pattern

```text
failed to create omt receiver for [%s]
```

## Parameters

- Argument 1 — OMT name: Requested output name or resolved full input source name.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to create omt receiver for [HOST (Camera)]
```

## Short description

OMT receiver creation failed.

## Trigger condition

Discovery resolves the source, but the OMT library returns no audio/video receiver.

## Explanation

The receive operation returns failure before setting preferred quality or capturing frames.

## Possible causes

- Discovery resolves the source, but the OMT library returns no audio/video receiver.

## What to check

1. Check the resolved source and OMT receiver initialization.

## Related log messages

- [OMT receiver created](log_d-9ec587925fbd7977.md)
