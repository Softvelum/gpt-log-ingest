# No usable audio or video track matches the request

Product: Nimble Streamer
Component: VOD transmuxing
Logging function: log_e
Log level: error
Documentation ID: log_e-26d8454a4dbceebf

## Log pattern

```text
file='%s' invalid video_index=%d audio_index=%d
```

## Parameters

- Argument 1 — Source media: Text filesystem path for local media, or source cache key for remote media.
- Argument 2 — Video selection: Integer video-track selection from the request or chunk name; 0 is the default selection.
- Argument 3 — Audio selection: Integer audio-track selection from the request or chunk name; 0 is the default selection.

## Log examples

Illustrative examples; these are not captured server logs.

```text
file='/media/movie.mp4' invalid video_index=9 audio_index=9
```

## Short description

No usable audio or video track matches the request.

## Trigger condition

Both requested track-existence checks fail while building a fragmented-MP4 playlist or aligning advertising subtitles.

## Explanation

The output request is canceled; subtitle-alignment handling first returns failure to its caller.

## Possible causes

- Both requested track-existence checks fail while building a fragmented-MP4 playlist or aligning advertising subtitles.

## What to check

1. Check track selections and supported codecs. Video support here requires AVC, HEVC, or AV1; audio checks recognize AAC, MP3, AC-3, or E-AC-3.

## Recommended action

Check track selections and supported codecs. Video support here requires AVC, HEVC, or AV1; audio checks recognize AAC, MP3, AC-3, or E-AC-3.
