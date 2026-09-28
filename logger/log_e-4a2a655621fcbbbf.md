# Playlist subtitle download is absent

Product: Nimble Streamer
Component: Server playlists
Logging function: log_e
Log level: error
Documentation ID: log_e-4a2a655621fcbbbf

## Log pattern

```text
subtitles url=[%s] doesn't exist
```

## Parameters

- Argument 1 — Subtitle location: Text identifying the remote subtitle file.

## Log examples

Illustrative examples; these are not captured server logs.

```text
subtitles url=[https://media.example/en.srt] doesn't exist
```

## Short description

Playlist subtitle download is absent.

## Trigger condition

A remote subtitle location has no corresponding cache item when media is opened.

## Explanation

The subtitle track is not loaded; media opening continues.

## Possible causes

- A remote subtitle location has no corresponding cache item when media is opened.

## What to check

1. Ensure that the subtitle download completed before playback.

## Recommended action

Ensure that the subtitle download completed before playback.
