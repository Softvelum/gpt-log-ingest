# EIT output buffer cannot grow

Product: Nimble Streamer
Component: Live media packaging and timed metadata
Logging function: log_e
Log level: error
Documentation ID: log_e-ac2c8bb80ede58c6

## Log pattern

```text
[EIT] cannot allocate mpeg2ts body for [%s/%s] used_size=%u frame_size=%u
```

## Parameters

- Argument 1 — Application: Application containing the live stream.
- Argument 2 — Stream: Live stream name.
- Argument 3 — Used size: Existing output buffer byte count.
- Argument 4 — Frame size: Incoming EIT payload size in bytes.

## Log examples

Illustrative examples; these are not captured server logs.

```text
[EIT] cannot allocate mpeg2ts body for [live/camera] used_size=1048576 frame_size=512
```

## Short description

EIT output buffer cannot grow.

## Trigger condition

Reserving output space fails while packaging EIT for MPEG-TS over HTTP.

## Explanation

The current metadata/subtitle packaging operation returns failure. The payload may already have been forwarded to other stream consumers before this failure.

## Possible causes

- Reserving output space fails while packaging EIT for MPEG-TS over HTTP.

## What to check

1. Check EIT payload size, accumulated transport-buffer bytes, and allocation limits.

## Recommended action

Resolve the output-buffer capacity problem.
