# HEVC decoder configuration cannot be built

Product: Nimble Streamer
Component: Live stream codec and packaging setup
Logging function: log_e
Log level: error
Documentation ID: log_e-2b8df0c9039ecbfb

## Log pattern

```text
[VM] set_video_codec_hevc function failed, buildHEVCDecoderConfigurationRecord call failed for [%s/%s]
```

## Parameters

- Argument 1 — Application: Application containing the affected live stream.
- Argument 2 — Stream: Live stream name.

## Log examples

Illustrative examples; these are not captured server logs.

```text
[VM] set_video_codec_hevc function failed, buildHEVCDecoderConfigurationRecord call failed for [live/camera]
```

## Short description

HEVC decoder configuration cannot be built.

## Trigger condition

Building the HEVC decoder configuration record from parsed parameters and the supplied parameter sets fails.

## Explanation

HEVC setup returns failure before publishing the new sequence header. The temporary record buffer has a 4 KiB maximum.

## Possible causes

- Building the HEVC decoder configuration record from parsed parameters and the supplied parameter sets fails.

## What to check

1. Check parameter-set contents and aggregate configuration size.

## Recommended action

Correct malformed or excessive HEVC configuration data.
