# Output chunk buffer cannot reserve required space

Product: Nimble Streamer
Component: Fragmented media to transport stream packaging
Logging function: log_e
Log level: error
Documentation ID: log_e-ee3fe6c5fc9eb741

## Log pattern

```text
cannot allocate buffer for chunk
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
cannot allocate buffer for chunk
```

## Short description

Output chunk buffer cannot reserve required space.

## Trigger condition

A live HLS/MPEG-TS chunk or a chunk reconstructed from fragmented media cannot reserve its initial tables, ID3 metadata, or additional audio/video frame space.

## Explanation

The current chunk initialization or reconstruction operation returns failure. Shared occurrences cover initial 376-byte transport tables, packed-audio metadata, and growth for video or audio samples; the message does not identify which operation failed.

## Possible causes

- A live HLS/MPEG-TS chunk or a chunk reconstructed from fragmented media cannot reserve its initial tables, ID3 metadata, or additional audio/video frame space.

## What to check

1. Check nearby stream context, input sample and metadata sizes, and memory or buffer limits.

## Recommended action

Resolve excessive chunk size or allocation pressure before retrying output generation.
