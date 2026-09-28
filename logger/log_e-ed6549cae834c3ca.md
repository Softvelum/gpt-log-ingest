# Requested media chunk name is invalid

Product: Nimble Streamer
Component: VOD transmuxing
Logging function: log_e
Log level: error
Documentation ID: log_e-ed6549cae834c3ca

## Log pattern

```text
invalid video chunk requested name='%s'
```

## Parameters

- Argument 1 — Chunk name: Text requested resource name rejected by its naming parser.

## Log examples

Illustrative examples; these are not captured server logs.

```text
invalid video chunk requested name='v_bad_0.fmp4'
```

## Short description

Requested media chunk name is invalid.

## Trigger condition

A MPEG-TS, fragmented-MP4 video/audio, or thumbnail request fails parsing of its required numeric chunk and track fields.

## Explanation

The output item is canceled and released before that chunk is generated; the fixed text also covers audio-only chunk names.

## Possible causes

- A MPEG-TS, fragmented-MP4 video/audio, or thumbnail request fails parsing of its required numeric chunk and track fields.

## What to check

1. Use the chunk names emitted by the matching generated playlist; check that numeric indexes and the resource suffix are preserved.

## Recommended action

Use the chunk names emitted by the matching generated playlist; check that numeric indexes and the resource suffix are preserved.
