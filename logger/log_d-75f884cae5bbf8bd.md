# SLDP selects a new advertising item

Product: Nimble Streamer
Component: SLDP playback and advertising
Logging function: log_d
Log level: debug
Documentation ID: log_d-75f884cae5bbf8bd

## Log pattern

```text
s=%d, video_sn=%d, audio_sn=%d play new content %s stream_time=%lld, start_time=%lld, offset=%lld

```

## Parameters

- Argument 1 — Socket: Connection descriptor reported by the playback transport.
- Argument 2 — Video number: Client-assigned video stream number for this playback selection, or its default value when no video is selected.
- Argument 3 — Audio number: Client-assigned audio stream number for this playback selection, or its default value when no audio is selected.
- Argument 4 — Content URI: Configured URI of the advertising media item.
- Argument 5 — Stream time: Current playback scheduling time in milliseconds.
- Argument 6 — Content start: Current stored start time for the newly selected content, in milliseconds; final alignment occurs later.
- Argument 7 — Offset: Current content playback offset in milliseconds.

## Log examples

Illustrative examples; these are not captured server logs.

```text
s=42, video_sn=1, audio_sn=2 play new content https://media.example/ad.mp4 stream_time=60000, start_time=0, offset=0

```

## Short description

SLDP selects a new advertising item.

## Trigger condition

A due content item has been selected from the pending schedule.

## Explanation

The message reports selection before loading and final media timestamp alignment. The newly selected item start value can still be its initial value at this point.

## Possible causes

- A due content item has been selected from the pending schedule.

## What to check

1. Check later loading and content-start messages to confirm playback.

## Recommended action

No action is required when the scheduled item is expected.
