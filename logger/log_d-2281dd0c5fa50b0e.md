# SLDP selection joins the last chosen advertising item

Product: Nimble Streamer
Component: SLDP playback and advertising
Logging function: log_d
Log level: debug
Documentation ID: log_d-2281dd0c5fa50b0e

## Log pattern

```text
s=%d, video_sn=%d, audio_sn=%d play last content %s stream_time=%lld, offset=%lld
```

## Parameters

- Argument 1 — Socket: Connection descriptor reported by the playback transport.
- Argument 2 — Video number: Client-assigned video stream number for this playback selection, or its default value when no video is selected.
- Argument 3 — Audio number: Client-assigned audio stream number for this playback selection, or its default value when no audio is selected.
- Argument 4 — Content URI: Configured URI of the advertising media item.
- Argument 5 — Stream time: Current playback scheduling time in milliseconds.
- Argument 6 — Offset: Playback offset into this advertising item in milliseconds.

## Log examples

Illustrative examples; these are not captured server logs.

```text
s=42, video_sn=1, audio_sn=2 play last content https://media.example/ad.mp4 stream_time=65000, offset=5000
```

## Short description

SLDP selection joins the last chosen advertising item.

## Trigger condition

A playback selection reaches the shared last-content time and has not yet consumed that item.

## Explanation

The last selected content is reused for this media selection. When offset-start is requested, it joins at the elapsed offset from the shared start.

## Possible causes

- A playback selection reaches the shared last-content time and has not yet consumed that item.

## What to check

1. Check synchronization of the video/audio selection numbers and the content offset.

## Recommended action

No action is required for coordinated advertising playback.
