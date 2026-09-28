# Packed-audio chunk cannot accept a sample

Product: Nimble Streamer
Component: Fragmented media to transport stream packaging
Logging function: log_e
Log level: error
Documentation ID: log_e-e84a9df0e79bc8ea

## Log pattern

```text
[AM] audio_buffer too small
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
[AM] audio_buffer too small
```

## Short description

Packed-audio chunk cannot accept a sample.

## Trigger condition

Writing an AAC ADTS header or the next audio sample into the packed-audio output buffer fails.

## Explanation

Audio-only output chunk reconstruction returns failure.

## Possible causes

- Writing an AAC ADTS header or the next audio sample into the packed-audio output buffer fails.

## What to check

1. Check accumulated chunk bytes and incoming audio sample size.

## Recommended action

Resolve excessive audio-chunk size or allocation pressure.
