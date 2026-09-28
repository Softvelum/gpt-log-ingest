# HEVC transport-header buffer cannot accept parameter sets

Product: Nimble Streamer
Component: Fragmented media to transport stream packaging
Logging function: log_e
Log level: error
Documentation ID: log_e-c5b23fa2b186a5b1

## Log pattern

```text
[VM] cannot allocate memory for mpeg2ts_hevc_header
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
[VM] cannot allocate memory for mpeg2ts_hevc_header
```

## Short description

HEVC transport-header buffer cannot accept parameter sets.

## Trigger condition

Writing a parameter-set payload or four-byte Annex B delimiter into the 4096-byte HEVC header buffer fails.

## Explanation

HEVC transport-header construction returns failure.

## Possible causes

- Writing a parameter-set payload or four-byte Annex B delimiter into the 4096-byte HEVC header buffer fails.

## What to check

1. Check total VPS, SPS, PPS and delimiter bytes against the header limit.

## Recommended action

Reduce oversized parameter-set data or resolve allocation failure.
