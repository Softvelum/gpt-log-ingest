# DVB subtitle writer exceeds its output buffer

Product: Nimble Streamer
Component: Live media packaging and timed metadata
Logging function: log_e
Log level: error
Documentation ID: log_e-daed9efcf54f9f09

## Log pattern

```text
[SUBTITLE] cannot write hls body for [%s/%s] used_size=%u frame_size=%u
```

## Parameters

- Argument 1 — Application: Application containing the live stream.
- Argument 2 — Stream: Live stream name.
- Argument 3 — Used size: Existing output buffer byte count.
- Argument 4 — Frame size: Incoming DVB subtitle payload size in bytes.

## Log examples

Illustrative examples; these are not captured server logs.

```text
[SUBTITLE] cannot write hls body for [live/camera] used_size=1048576 frame_size=512
```

## Short description

DVB subtitle writer exceeds its output buffer.

## Trigger condition

The transport packet writer reports insufficient capacity while packaging DVB subtitle for HLS.

## Explanation

The current metadata/subtitle packaging operation returns failure. The payload may already have been forwarded to other stream consumers before this failure.

## Possible causes

- The transport packet writer reports insufficient capacity while packaging DVB subtitle for HLS.

## What to check

1. Check DVB subtitle payload size, accumulated transport-buffer bytes, and allocation limits.

## Recommended action

Resolve the output-buffer capacity problem.
