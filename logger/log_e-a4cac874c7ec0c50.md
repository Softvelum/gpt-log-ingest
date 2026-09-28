# Caption SEI output buffer cannot grow

Product: Nimble Streamer
Component: Live media packaging and timed metadata
Logging function: log_e
Log level: error
Documentation ID: log_e-a4cac874c7ec0c50

## Log pattern

```text
failed to alloc avcc buffer to write sei nalu for [%s/%s], used_size=%u, sei_size=%d
```

## Parameters

- Argument 1 — Application: Application containing the live stream.
- Argument 2 — Stream: Live stream name.
- Argument 3 — Used size: Existing rewritten AVC buffer bytes.
- Argument 4 — SEI size: Intended generated SEI size in bytes; the argument uses the platform size type while the format requests int.

## Log examples

No portable exact example is asserted: the SEI-size value uses the platform size type with %d, which mismatches on the reviewed 64-bit target.

## Short description

Caption SEI output buffer cannot grow.

## Trigger condition

The AVC caption rewriter cannot reserve space for a four-byte NAL length and generated caption SEI data.

## Explanation

Caption rewriting fails; the calling video path retains the original frame. The SEI-size argument has a platform-dependent formatting mismatch.

## Possible causes

- The AVC caption rewriter cannot reserve space for a four-byte NAL length and generated caption SEI data.

## What to check

1. Check caption SEI size, existing rewritten bytes, and allocation limits.

## Recommended action

Resolve the caption-buffer allocation failure.
