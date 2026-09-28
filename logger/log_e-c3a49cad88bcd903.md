# SDI output is recreated when audio arrives late

Product: Nimble Streamer
Component: SDI media integration
Logging function: log_e
Log level: error
Documentation ID: log_e-c3a49cad88bcd903

## Log pattern

```text
reset sdi output. later audio arrived [%s/%s]
```

## Parameters

- Argument 1 — Device type: Configured media-device driver type.
- Argument 2 — Device stream: Configured SDI device or stream name.

## Log examples

Illustrative examples; these are not captured server logs.

```text
reset sdi output. later audio arrived [decklink/SDI output]
```

## Short description

SDI output is recreated when audio arrives late.

## Trigger condition

Video output already exists when the first audio configuration arrives.

## Explanation

The device encoder is reset and recreated with audio. If recreation succeeds, the current audio frame is sent; failure stops message processing.

## Possible causes

- The first audio frame arrives after the video-only output device was created.

## What to check

1. Check whether video-only output started before the source supplied audio.

## Related log messages

- [SDI output audio parameters changed](log_e-c69463bf5772c6f4.md)
