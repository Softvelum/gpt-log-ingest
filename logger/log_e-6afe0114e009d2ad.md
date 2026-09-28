# DVR video sample is invalid for MPEG-TS accounting

Product: Nimble Streamer
Component: DVR recording
Logging function: log_e
Log level: error
Documentation ID: log_e-6afe0114e009d2ad

## Log pattern

```text
[VM] wrong sample
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
[VM] wrong sample
```

## Short description

DVR video sample is invalid for MPEG-TS accounting.

## Trigger condition

AVC, HEVC, or MPEG-2 sample processing raises a MPEG-TS exception while calculating continuity information.

## Explanation

The recording pass returns failure.

## Possible causes

- AVC, HEVC, or MPEG-2 sample processing raises a MPEG-TS exception while calculating continuity information.

## What to check

1. Check the incoming compressed video sample boundaries and contents.

## Recommended action

Check the incoming compressed video sample boundaries and contents.
