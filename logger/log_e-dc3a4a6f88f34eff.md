# Scheduled SCTE-35 marker failed during file playback

Product: Nimble Streamer
Component: Server playlists
Logging function: log_e
Log level: error
Documentation ID: log_e-dc3a4a6f88f34eff

## Log pattern

```text
ServerBlockProcessor: inject scte35 marker failed for file [%s], stream=[%s/%s]
```

## Parameters

- Argument 1 — Source media: Text identifying the source media file or URI.
- Argument 2 — Application: Text naming the application.
- Argument 3 — Stream: Text naming the stream.

## Log examples

Illustrative examples; these are not captured server logs.

```text
ServerBlockProcessor: inject scte35 marker failed for file [/media/program.mp4], stream=[live/camera]
```

## Short description

Scheduled SCTE-35 marker failed during file playback.

## Trigger condition

A due wall-clock or media-offset marker cannot be submitted to the output stream.

## Explanation

Media processing continues; the failed marker remains pending rather than being removed.

## Possible causes

- A due wall-clock or media-offset marker cannot be submitted to the output stream.

## What to check

1. Check that the output stream accepts SCTE-35 injection and inspect preceding injection errors.

## Recommended action

Check that the output stream accepts SCTE-35 injection and inspect preceding injection errors.
