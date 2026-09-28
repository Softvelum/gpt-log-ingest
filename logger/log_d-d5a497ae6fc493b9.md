# SLDP advertising media finishes loading

Product: Nimble Streamer
Component: SLDP playback and advertising
Logging function: log_d
Log level: debug
Documentation ID: log_d-d5a497ae6fc493b9

## Log pattern

```text
s=%d, video_sn=%d, audio_sn=%d %s content loaded
```

## Parameters

- Argument 1 — Socket: Connection descriptor reported by the playback transport.
- Argument 2 — Video number: Client-assigned video stream number for this playback selection, or its default value when no video is selected.
- Argument 3 — Audio number: Client-assigned audio stream number for this playback selection, or its default value when no audio is selected.
- Argument 4 — Content URI: Configured URI of the advertising media item.

## Log examples

Illustrative examples; these are not captured server logs.

```text
s=42, video_sn=1, audio_sn=2 https://media.example/ad.mp4 content loaded
```

## Short description

SLDP advertising media finishes loading.

## Trigger condition

The selected item becomes ready in a matching content-load callback while playback is waiting.

## Explanation

The observer and timeout are removed and the sender attempts to continue loading and playing the content.

## Possible causes

- The selected item becomes ready in a matching content-load callback while playback is waiting.

## What to check

1. Check subsequent reader and content-start messages.

## Recommended action

No action is required for a successful load.
