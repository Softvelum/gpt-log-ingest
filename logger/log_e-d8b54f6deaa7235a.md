# Live audio sound format is unsupported

Product: Nimble Streamer
Component: Audio decoding
Logging function: log_e
Log level: error
Documentation ID: log_e-d8b54f6deaa7235a

## Log pattern

```text
unsupported sound format %d [%s/%s]
```

## Parameters

- Argument 1 — Sound format: Integer sound-format identifier; the decoder reports a mapped FFmpeg codec identifier, while compressed forwarding reports the four-bit RTMP sound format.
- Argument 2 — Application: Application name, text.
- Argument 3 — Stream: Stream name, text.

Decoder dispatch checks mapped codec IDs. Encoder forwarding checks the original RTMP header nibble, so identical numbers have different meanings between occurrences.

## Log examples

Illustrative examples; these are not captured server logs.

```text
unsupported sound format 0 [live/camera]
```

## Short description

Live audio sound format is unsupported.

## Trigger condition

Ordinary live audio dispatch cannot match the sound format to AAC, MP3, or Speex.

## Explanation

The packet is rejected and the decoding or compressed-audio forwarding operation fails.

## Possible causes

- The ordinary audio packet header identifies an unsupported format.

## What to check

1. Check the packet sound header and selected input codec.

## Recommended action

Use a supported ordinary audio format or its proper dedicated packet type.
