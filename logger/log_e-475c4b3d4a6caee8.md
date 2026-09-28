# Scenario audio codec is unsupported

Product: Nimble Streamer
Component: Scenario transcoder processes
Logging function: log_e
Log level: error
Documentation ID: log_e-475c4b3d4a6caee8

## Log pattern

```text
unsupported audio codec=%d stream=%s/%s scenario=%s
```

## Parameters

- Argument 1 — Codec: Numeric audio codec identifier, integer; no unit.
- Argument 2 — Application: Application name, text.
- Argument 3 — Stream: Stream name, text.
- Argument 4 — Scenario: Transcoding scenario identifier, text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
unsupported audio codec=999 stream=live/camera scenario=scenario1
```

## Short description

Scenario audio codec is unsupported.

## Trigger condition

The child sends audio codec information with an identifier outside the supported set.

## Explanation

The codec message is rejected, connection processing fails and the scenario controller is removed. Supported audio codecs on this message path are AAC, AC-3, E-AC-3, MP3, Opus, PCMA and PCMU.

## Possible causes

- The child announces an unsupported or invalid audio codec identifier.

## What to check

1. Check the reported codec and consistency of the parent and scenario transcoder installations.

## Recommended action

Select a supported output codec and use matching parent and child versions.
