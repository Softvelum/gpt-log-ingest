# Encoded NAL-unit conversion failed

Product: Nimble Streamer
Component: Encoded video packet conversion
Logging function: log_e
Log level: error
Documentation ID: log_e-2f7befe28fb720cc

## Log pattern

```text
failed to convert annexb buffer to avcc
```

## Parameters

The ordinary converter preserves NAL payloads while replacing start codes with lengths. The NVIDIA timing variant also combines its generated timing information with the forwarded clock timestamp.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to convert annexb buffer to avcc
```

## Short description

Encoded NAL-unit conversion failed.

## Trigger condition

Converting encoder Annex B output into length-prefixed NAL units fails. Native NVIDIA H.264 may additionally merge forwarded picture-timing timecodes.

## Explanation

NVIDIA and VAAPI frame writes return failure immediately. SVT HEVC releases the encoded packet, clears the failed conversion buffer and then returns failure without writing it; the SVT occurrence also appears during final flushing.

## Possible causes

- The output buffer cannot grow to hold the length-prefixed data.
- A VAAPI packet segment is too short.
- NVIDIA timecode merging encounters truncated timing data, unsupported picture structure or unexpected timing-message order.

## What to check

1. Check nearby encoder errors, output-buffer size/resource limits and whether NVIDIA SEI timecode forwarding is enabled.

## Recommended action

Correct the packet or buffer failure; if the NVIDIA timing variant is responsible, use supported timecode data or disable forwarding when unnecessary.
