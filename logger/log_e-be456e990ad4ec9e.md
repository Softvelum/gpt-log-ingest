# HEVC transport header exceeds its buffer

Product: Nimble Streamer
Component: Live stream codec and packaging setup
Logging function: log_e
Log level: error
Documentation ID: log_e-be456e990ad4ec9e

## Log pattern

```text
[VM] cannot allocate memory for mpeg2ts_avc_header
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
[VM] cannot allocate memory for mpeg2ts_avc_header
```

## Short description

HEVC transport header exceeds its buffer.

## Trigger condition

Writing HEVC parameter sets or Annex B delimiters into the fixed 4 KiB transport-header buffer fails.

## Explanation

HEVC setup returns failure before the decoder configuration and packaging are completed.

## Possible causes

- Writing HEVC parameter sets or Annex B delimiters into the fixed 4 KiB transport-header buffer fails.

## What to check

1. Check aggregate HEVC parameter-set size and memory allocation errors.

## Recommended action

Correct oversized parameter sets or resolve the header-buffer allocation failure.
