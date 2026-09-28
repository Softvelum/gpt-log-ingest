# VOD output resource type is unsupported

Product: Nimble Streamer
Component: VOD transmuxing
Logging function: log_e
Log level: error
Documentation ID: log_e-e9e3ce0aa82b00d5

## Log pattern

```text
invalid request to transmux chunk_name='%s'
```

## Parameters

- Argument 1 — Requested resource: Text output resource name that did not match a supported request branch.

## Log examples

Illustrative examples; these are not captured server logs.

```text
invalid request to transmux chunk_name='unsupported.bin'
```

## Short description

VOD output resource type is unsupported.

## Trigger condition

The resource matches none of the supported playlist, subtitle, MPEG-TS, fragmented-MP4, or DASH request forms.

## Explanation

The requested output cache item is canceled and released; this attempt produces no usable output.

## Possible causes

- The resource matches none of the supported playlist, subtitle, MPEG-TS, fragmented-MP4, or DASH request forms.

## What to check

1. Request a supported output resource produced by the corresponding manifest or playlist.

## Recommended action

Request a supported output resource produced by the corresponding manifest or playlist.
