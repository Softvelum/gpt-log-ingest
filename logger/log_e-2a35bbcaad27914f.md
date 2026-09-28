# HEVC parameter-set collection is incomplete

Product: Nimble Streamer
Component: Live stream codec and packaging setup
Logging function: log_e
Log level: error
Documentation ID: log_e-2a35bbcaad27914f

## Log pattern

```text
[VM] set_video_codec_hevc function, vps/sps/pps empty for [%s/%s]
```

## Parameters

- Argument 1 — Application: Application containing the affected live stream.
- Argument 2 — Stream: Live stream name.

## Log examples

Illustrative examples; these are not captured server logs.

```text
[VM] set_video_codec_hevc function, vps/sps/pps empty for [live/camera]
```

## Short description

HEVC parameter-set collection is incomplete.

## Trigger condition

At least one of the video, sequence, or picture parameter-set collections is empty.

## Explanation

HEVC setup fails before configuring the stream.

## Possible causes

- At least one of the video, sequence, or picture parameter-set collections is empty.

## What to check

1. Check that the input provides VPS, SPS, and PPS data.

## Recommended action

Send all required HEVC parameter sets.
