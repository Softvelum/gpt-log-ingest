# HEVC decoder configuration cannot be split

Product: Nimble Streamer
Component: Fragmented media to transport stream packaging
Logging function: log_e
Log level: error
Documentation ID: log_e-445e602f4f9eccc0

## Log pattern

```text
failed to split hevc_decoder_record
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to split hevc_decoder_record
```

## Short description

HEVC decoder configuration cannot be split.

## Trigger condition

The decoder-configuration record cannot be parsed into its HEVC parameter-set collections.

## Explanation

HEVC transport-header construction fails before copying parameter sets.

## Possible causes

- The decoder-configuration record cannot be parsed into its HEVC parameter-set collections.

## What to check

1. Check the HEVC decoder-configuration lengths and record structure.

## Recommended action

Provide a complete valid HEVC configuration record.
