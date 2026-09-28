# Explicit audio packet changed media type

Product: Nimble Streamer
Component: MPEG-TS live stream selection
Logging function: log_d
Log level: debug
Documentation ID: log_d-971cf29fefdd942b

## Log pattern

```text
updateAvailable audio type changed for audio_pid=%d from %d to %d
```

## Parameters

- Argument 1 — packet\_id: MPEG-TS packet identifier printed as a decimal integer.
- Argument 2 — previous\_type: Previously selected stream-type code in decimal.
- Argument 3 — new\_type: New advertised stream-type code in decimal.

## Log examples

Illustrative examples; these are not captured server logs.

```text
updateAvailable audio type changed for audio_pid=256 from 15 to 17
```

## Short description

Explicit audio packet changed media type.

## Trigger condition

An explicitly configured audio packet identifier is present, but its new type differs from the previous nonzero type.

## Explanation

Audio codec initialization, timestamps, AAC header state, and the audio frame buffer are reset. The same configured identifier remains selected and its advertised type is updated. For LATM audio with probing enabled, the actual audio type is left unset until payload probing.
