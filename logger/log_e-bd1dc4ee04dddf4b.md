# Audio-only HLS part cannot reserve its initial metadata

Product: Nimble Streamer
Component: Live media packaging and timed metadata
Logging function: log_e
Log level: error
Documentation ID: log_e-bd1dc4ee04dddf4b

## Log pattern

```text
cannot allocate buffer for audio part
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
cannot allocate buffer for audio part
```

## Short description

Audio-only HLS part cannot reserve its initial metadata.

## Trigger condition

Low-latency audio-only HLS needs a new part, and its buffer cannot reserve the ID3 timestamp prefix and current ID3 metadata.

## Explanation

Part initialization fails and the caller cannot append this audio part.

## Possible causes

- Low-latency audio-only HLS needs a new part, and its buffer cannot reserve the ID3 timestamp prefix and current ID3 metadata.

## What to check

1. Check the amount of queued ID3 metadata and audio-part allocation limits.

## Recommended action

Resolve the metadata or buffer-capacity problem.
