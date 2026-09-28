# DVR export begins parsing video and audio

Product: Nimble Streamer
Component: DVR export
Logging function: log_d
Log level: debug
Documentation ID: log_d-ea8eaae87640c4af

## Log pattern

```text
parsing %llu/%llu video/audio segments for '%s' start=%llu end=%llu
```

## Parameters

- Argument 1 — Video count: Unsigned number of video segments.
- Argument 2 — Audio count: Unsigned number of audio segments.
- Argument 3 — Stream: Text naming the application and stream.
- Argument 4 — UTC start: 64-bit requested UTC start in seconds since the Unix epoch; zero leaves this bound unspecified.
- Argument 5 — UTC end: 64-bit requested UTC end in seconds since the Unix epoch; zero leaves this bound unspecified.

## Log examples

Illustrative examples; these are not captured server logs.

```text
parsing 10/10 video/audio segments for 'live/camera' start=1700000000 end=1700000060
```

## Short description

DVR export begins parsing video and audio.

## Trigger condition

The requested range contains at least one video segment.

## Explanation

Export enters incremental parsing before building a downloadable MP4.

## Possible causes

- The requested range contains at least one video segment.
