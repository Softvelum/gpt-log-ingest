# SLDP aligns audio-only advertising timestamps

Product: Nimble Streamer
Component: SLDP playback and advertising
Logging function: log_d
Log level: debug
Documentation ID: log_d-eea57ac84b1d8bdf

## Log pattern

```text
s=%d, video_sn=%d, audio_sn=%d %s content start, video only, initial_audio_timestamp=%lld, offset=%lld
```

## Parameters

- Argument 1 — Socket: Connection descriptor reported by the playback transport.
- Argument 2 — Video number: Client-assigned video stream number for this playback selection, or its default value when no video is selected.
- Argument 3 — Audio number: Client-assigned audio stream number for this playback selection, or its default value when no audio is selected.
- Argument 4 — Content URI: Configured URI of the advertising media item.
- Argument 5 — Initial audio timestamp: Raw insertion timestamp in the associated media timestamp scale, which is not printed.
- Argument 6 — Offset: Playback position within the advertisement in milliseconds.

The example assumes a 1,000-tick-per-second insertion timestamp. Other media timescales produce different raw values.

## Log examples

Illustrative examples; these are not captured server logs.

```text
s=42, video_sn=0, audio_sn=2 https://media.example/ad.mp4 content start, video only, initial_audio_timestamp=120000, offset=0
```

## Short description

SLDP aligns audio-only advertising timestamps.

## Trigger condition

A ready audio-only advertising reader is assigned the stream insertion timestamp.

## Explanation

The reader is prepared before requesting its first content item. The printed video only wording is incorrect for this audio-only branch.

## Possible causes

- A ready audio-only advertising reader is assigned the stream insertion timestamp.

## What to check

1. Check the media timescale before interpreting the raw starting timestamp.

## Recommended action

No action is required for an expected advertising start.
