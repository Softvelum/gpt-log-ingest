# MPEG-2 video fragment cannot be reconstructed

Product: Nimble Streamer
Component: Fragmented media to transport stream packaging
Logging function: log_e
Log level: error
Documentation ID: log_e-17efdedd2026b5a5

## Log pattern

```text
cannot load mpeg2video segment
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
cannot load mpeg2video segment
```

## Short description

MPEG-2 video fragment cannot be reconstructed.

## Trigger condition

An MP4-structure error occurs while reading the MPEG-2 video media fragment or its sample information.

## Explanation

Video-only transport chunk reconstruction returns failure. For AVC and HEVC, initialization headers were already processed before this protected fragment-reading operation.

## Possible causes

- An MP4-structure error occurs while reading the MPEG-2 video media fragment or its sample information.

## What to check

1. Check the fragment boxes, sample tables, and payload lengths.

## Recommended action

Provide a complete media fragment matching its initialization.
