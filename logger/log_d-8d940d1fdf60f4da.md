# Automatic audio packet changed media type

Product: Nimble Streamer
Component: MPEG-TS live stream selection
Logging function: log_d
Log level: debug
Documentation ID: log_d-8d940d1fdf60f4da

## Log pattern

```text
updateAvailable audio type changed for actual_audio_pid=%d from %d to %d
```

## Parameters

- Argument 1 — packet\_id: MPEG-TS packet identifier printed as a decimal integer.
- Argument 2 — previous\_type: Previously selected stream-type code in decimal.
- Argument 3 — new\_type: New advertised stream-type code in decimal.

## Log examples

Illustrative examples; these are not captured server logs.

```text
updateAvailable audio type changed for actual_audio_pid=256 from 15 to 17
```

## Short description

Automatic audio packet changed media type.

## Trigger condition

The active automatically selected audio identifier still equals the current default, but its advertised stream type changed.

## Explanation

Audio codec initialization, timestamps, AAC header state, and the audio frame buffer are reset. The new type is retained and the identifier remains requested. For LATM audio with probing enabled, the actual audio type is left unset until payload probing.
