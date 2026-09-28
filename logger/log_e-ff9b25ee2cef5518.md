# Audio packet codec differs from the active decoder

Product: Nimble Streamer
Component: Audio decoding
Logging function: log_e
Log level: error
Documentation ID: log_e-ff9b25ee2cef5518

## Log pattern

```text
wrong sound format %d %d [%s/%s]
```

## Parameters

- Argument 1 — Incoming codec: FFmpeg codec identifier inferred from the RTMP sound header, integer; unknown headers map to no codec.
- Argument 2 — Expected codec: Configured decoder FFmpeg codec identifier, integer.
- Argument 3 — Application: Application name, text.
- Argument 4 — Stream: Stream name, text.

## Log examples

Illustrative examples; these are not captured server logs.

```text
wrong sound format 86017 86018 [live/camera]
```

## Short description

Audio packet codec differs from the active decoder.

## Trigger condition

The packet sound header maps to a codec different from the configured decoder codec.

## Explanation

The packet is rejected and audio processing fails.

## Possible causes

- The publisher changed audio codec or supplied an unexpected sound-format header.

## What to check

1. Compare the incoming and expected FFmpeg codec identifiers and the latest sequence header.

## Recommended action

Keep packet headers and decoder configuration consistent or restart after a codec change.
