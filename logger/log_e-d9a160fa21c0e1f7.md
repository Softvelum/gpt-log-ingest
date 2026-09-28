# Reconstructed audio exceeds its transport buffer

Product: Nimble Streamer
Component: Fragmented media to transport stream packaging
Logging function: log_e
Log level: error
Documentation ID: log_e-d9a160fa21c0e1f7

## Log pattern

```text
[AM] cannot write mpeg2ts body
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
[AM] cannot write mpeg2ts body
```

## Short description

Reconstructed audio exceeds its transport buffer.

## Trigger condition

An audio transport writer reports insufficient capacity while interleaving audio with AVC, HEVC, or MPEG-2 video.

## Explanation

Combined audio/video output chunk reconstruction returns failure. AAC paths add an ADTS header; other accepted audio is written without that header.

## Possible causes

- An audio transport writer reports insufficient capacity while interleaving audio with AVC, HEVC, or MPEG-2 video.

## What to check

1. Check audio sample size and transport overhead against available output capacity.

## Recommended action

Resolve the output-buffer sizing problem.
