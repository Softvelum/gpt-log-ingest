# DVR HEVC initialization exceeds its buffer

Product: Nimble Streamer
Component: DVR recording
Logging function: log_e
Log level: error
Documentation ID: log_e-18fdbb19cc235b1f

## Log pattern

```text
DashLiveSegmentList::generate_h265_initialization_segment failed to generate init segment. hevc_codec_info_size=%d
```

## Parameters

- Argument 1 — Codec data bytes: Size in bytes of the HEVC configuration record.

## Log examples

Illustrative examples; these are not captured server logs.

```text
DashLiveSegmentList::generate_h265_initialization_segment failed to generate init segment. hevc_codec_info_size=65536
```

## Short description

DVR HEVC initialization exceeds its buffer.

## Trigger condition

The calculated MP4 initialization size is greater than the fixed initialization buffer.

## Explanation

The recording initialization returns failure.

## Possible causes

- The calculated MP4 initialization size is greater than the fixed initialization buffer.

## What to check

1. Check unusually large codec configuration data.

## Recommended action

Check unusually large codec configuration data.
