# DVR export begins parsing audio

Product: Nimble Streamer
Component: DVR export
Logging function: log_d
Log level: debug
Documentation ID: log_d-a3c9ec53957cdf84

## Log pattern

```text
parsing %llu audio segments for '%s' start=%llu end=%llu
```

## Parameters

- Argument 1 — Audio count: Unsigned number of audio segments.
- Argument 2 — Stream: Text naming the application and stream.
- Argument 3 — UTC start: 64-bit requested UTC start in seconds since the Unix epoch; zero leaves this bound unspecified.
- Argument 4 — UTC end: 64-bit requested UTC end in seconds since the Unix epoch; zero leaves this bound unspecified.

## Log examples

Illustrative examples; these are not captured server logs.

```text
parsing 10 audio segments for 'live/camera' start=1700000000 end=1700000060
```

## Short description

DVR export begins parsing audio.

## Trigger condition

The requested range has audio segments and no video segments.

## Explanation

Export enters audio parsing before building an audio-only MP4.

## Possible causes

- The requested range has audio segments and no video segments.
