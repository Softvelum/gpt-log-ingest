# OMT SEI-timecode worker created

Product: Nimble Streamer
Component: OMT media integration
Logging function: log_d
Log level: debug
Documentation ID: log_d-748ba98ba89bf2ab

## Log pattern

```text
omt sei sender thread created for [%s]
```

## Parameters

- Argument 1 — OMT name: Requested output name or resolved full input source name.

## Log examples

Illustrative examples; these are not captured server logs.

```text
omt sei sender thread created for [Output]
```

## Short description

OMT SEI-timecode worker created.

## Trigger condition

SEI-timecode output successfully creates its worker after creating the OMT sender.

## Explanation

Message processing proceeds to dispatch video, audio, or timecode data through the configured output mode.

## Possible causes

- Successful initialization of scheduled OMT output.

## Related log messages

- [OMT sender created](log_d-b089f98d3c14a8be.md)
