# DVR export cannot create audio silence

Product: Nimble Streamer
Component: DVR export
Logging function: log_e
Log level: error
Documentation ID: log_e-e651cff81379ea3b

## Log pattern

```text
failed init silence sample, stream=%s utc_start=%lld utc_end=%lld
```

## Parameters

- Argument 1 — Stream: Text naming the application and stream.
- Argument 2 — UTC start: 64-bit requested UTC start in seconds since the Unix epoch; zero leaves this bound unspecified.
- Argument 3 — UTC end: 64-bit requested UTC end in seconds since the Unix epoch; zero leaves this bound unspecified.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed init silence sample, stream=live/camera utc_start=1700000000 utc_end=1700000060
```

## Short description

DVR export cannot create audio silence.

## Trigger condition

Combined export cannot create its initial silence sample from the audio codec configuration.

## Explanation

Preparation fails and the export closes; silence is needed for track alignment.

## Possible causes

- Combined export cannot create its initial silence sample from the audio codec configuration.

## What to check

1. Check AAC configuration and channel support or MP3 silence generation; other codecs do not have a silence path here.

## Recommended action

Check AAC configuration and channel support or MP3 silence generation; other codecs do not have a silence path here.
