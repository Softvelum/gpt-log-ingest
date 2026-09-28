# RTMP HEVC decoder configuration cannot be separated

Product: Nimble Streamer
Component: RTMP media input
Logging function: log_e
Log level: error
Documentation ID: log_e-b48ba399c4a9aa45

## Log pattern

```text
[VM] splitHEVCDecoderConfigurationRecord
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
[VM] splitHEVCDecoderConfigurationRecord
```

## Short description

RTMP HEVC decoder configuration cannot be separated.

## Trigger condition

The legacy HEVC sequence-header decoder configuration cannot be parsed into its parameter sets.

## Explanation

The video handler fails before HEVC initialization and connection processing stops.

## Possible causes

- The legacy HEVC sequence-header decoder configuration cannot be parsed into its parameter sets.

## What to check

1. Check the completeness and structure of the HEVC decoder configuration record.

## Recommended action

Send a valid HEVC sequence header with its required parameter sets.
