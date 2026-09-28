# H.265 live-stream codec setup failed

Product: Nimble Streamer
Component: RTSP input and RTP media
Logging function: log_e
Log level: error
Documentation ID: log_e-a54e57d70a8b3b9e

## Log pattern

```text
H265: unable to set codec params
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
H265: unable to set codec params
```

## Short description

H.265 live-stream codec setup failed.

## Trigger condition

The live-stream H.265 configuration operation rejects the collected video, sequence, or picture parameter sets.

## Explanation

RTSP H.265 initialization returns failure. The downstream operation can reject an ended stream, invalid parameter data, or an output-header preparation failure.

## Possible causes

- The live-stream H.265 configuration operation rejects the collected video, sequence, or picture parameter sets.

## What to check

1. Check the H.265 parameter sets and any accompanying codec-configuration errors.
