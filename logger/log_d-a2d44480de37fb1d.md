# SLDP playback waits for advertising media

Product: Nimble Streamer
Component: SLDP playback and advertising
Logging function: log_d
Log level: debug
Documentation ID: log_d-a2d44480de37fb1d

## Log pattern

```text
s=%d, video_sn=%d, audio_sn=%d waiting %d sec for content %s
```

## Parameters

- Argument 1 — Socket: Connection descriptor reported by the playback transport.
- Argument 2 — Video number: Client-assigned video stream number for this playback selection, or its default value when no video is selected.
- Argument 3 — Audio number: Client-assigned audio stream number for this playback selection, or its default value when no audio is selected.
- Argument 4 — Wait: Content-load timeout in seconds.
- Argument 5 — Content URI: Configured URI of the advertising media item.

## Log examples

Illustrative examples; these are not captured server logs.

```text
s=42, video_sn=1, audio_sn=2 waiting 5 sec for content https://media.example/ad.mp4
```

## Short description

SLDP playback waits for advertising media.

## Trigger condition

Unready content is remote, has a nonempty URI, and has a positive configured wait timeout.

## Explanation

A load observer and timer are registered, playback enters a cache-wait state, and output is disabled until load completion or timeout.

## Possible causes

- Unready content is remote, has a nonempty URI, and has a positive configured wait timeout.

## What to check

1. Check remote download progress and the configured wait period.

## Recommended action

Ensure the advertisement can load within the allowed wait.
