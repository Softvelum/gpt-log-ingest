# OMT sender created

Product: Nimble Streamer
Component: OMT media integration
Logging function: log_d
Log level: debug
Documentation ID: log_d-b089f98d3c14a8be

## Log pattern

```text
omt sender created for [%s]
```

## Parameters

- Argument 1 — OMT name: Requested output name or resolved full input source name.

## Log examples

Illustrative examples; these are not captured server logs.

```text
omt sender created for [Output]
```

## Short description

OMT sender created.

## Trigger condition

The OMT runtime successfully creates the sender required for outgoing messages.

## Explanation

The sender is retained for later messages. In SEI-timecode mode the scheduled worker still needs to be created afterward.

## Possible causes

- Normal lazy initialization of OMT output.

## Related log messages

- [OMT SEI-timecode worker created](log_d-748ba98ba89bf2ab.md)
- [OMT sender creation failed](log_e-789144cb7e3a1e37.md)
