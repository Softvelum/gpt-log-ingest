# HLS input rejected an ended playlist

Product: Nimble Streamer
Component: HLS and MPEG-TS HTTP input
Logging function: log_e
Log level: error
Documentation ID: log_e-d3abc36d58456c9a

## Log pattern

```text
pulling HLS VOD stream url='%s' is not supported
```

## Parameters

- Argument 1 — original\_url: Original configured HLS input URL, which can differ from a redirected playlist URL.

## Log examples

Illustrative examples; these are not captured server logs.

```text
pulling HLS VOD stream url='https://origin.example/movie.m3u8' is not supported
```

## Short description

HLS input rejected an ended playlist.

## Trigger condition

Playlist parsing encountered the end-of-list tag.

## Explanation

The input treats the playlist as VOD, requests shutdown, and returns. This live pulling path does not accept an ended HLS playlist.

## Possible causes

- The playlist contains an end-of-list tag.

## What to check

1. Check whether the source is an ended or VOD playlist.

## Recommended action

Provide a supported ongoing live playlist for this input.
