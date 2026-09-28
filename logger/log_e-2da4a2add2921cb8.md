# SVT HEVC output packet retrieval failed

Product: Nimble Streamer
Component: SVT HEVC encoding
Logging function: log_e
Log level: error
Documentation ID: log_e-2da4a2add2921cb8

## Log pattern

```text
failed to get packet, err=%d
```

## Parameters

- Argument 1 — Encoder status: SVT HEVC result code, integer; no unit.

One occurrence retrieves available packets after a picture; the other drains packets after sending end of stream.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to get packet, err=-1
```

## Short description

SVT HEVC output packet retrieval failed.

## Trigger condition

Retrieving an encoded packet returns an error other than the normal empty-queue result.

## Explanation

The current frame write or final flush returns failure; remaining encoded output is not drained by this call.

## Possible causes

- SVT HEVC fails while returning encoded output.

## What to check

1. Check the SVT status and preceding encoding errors.

## Recommended action

Correct the encoder failure and restart encoding if necessary.
