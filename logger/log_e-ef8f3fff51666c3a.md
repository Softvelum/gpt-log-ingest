# DVR DVB subtitles timestamp moved backward

Product: Nimble Streamer
Component: DVR recording
Logging function: log_e
Log level: error
Documentation ID: log_e-ef8f3fff51666c3a

## Log pattern

```text
start new period on dvb subtitles segment overrun stream='%s' prev_start_time=%llu start_time=%llu
```

## Parameters

- Argument 1 — Stream: Text naming the application and stream.
- Argument 2 — Previous start: Unsigned 64-bit previous segment start in the media clock ticks.
- Argument 3 — New start: Unsigned 64-bit new segment start in the same media clock ticks.

For DVB subtitles both timestamps use 90000-Hz presentation ticks.

## Log examples

Illustrative examples; these are not captured server logs.

```text
start new period on dvb subtitles segment overrun stream='live/camera' prev_start_time=180000 start_time=90000
```

## Short description

DVR DVB subtitles timestamp moved backward.

## Trigger condition

Invalid-segment dropping is enabled and a new DVB subtitles segment starts before the preceding start timestamp.

## Explanation

Recording state resets, a new archive period starts, and the current item is retained for processing again.

## Possible causes

- Invalid-segment dropping is enabled and a new DVB subtitles segment starts before the preceding start timestamp.

## What to check

1. Check the input timestamp reset and period transition.

## Recommended action

Check the input timestamp reset and period transition.
