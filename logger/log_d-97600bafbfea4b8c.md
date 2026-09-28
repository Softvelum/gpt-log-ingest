# DVR combined audio/video export sends audio

Product: Nimble Streamer
Component: DVR export
Logging function: log_d
Log level: debug
Documentation ID: log_d-97600bafbfea4b8c

## Log pattern

```text
sending audio segment: period=%d start_time=%lld size=%d stream=%s utc_start=%lld utc_end=%lld
```

## Parameters

- Argument 1 — Recording period: Integer identifier of the continuous recording period.
- Argument 2 — Segment start: 64-bit segment start timestamp in that track's media timescale.
- Argument 3 — Media bytes: Number of media sample bytes selected for transmission.
- Argument 4 — Stream: Text naming the application and stream.
- Argument 5 — UTC start: 64-bit requested UTC start in seconds since the Unix epoch; zero leaves this bound unspecified.
- Argument 6 — UTC end: 64-bit requested UTC end in seconds since the Unix epoch; zero leaves this bound unspecified.

## Log examples

Illustrative examples; these are not captured server logs.

```text
sending audio segment: period=1 start_time=90000 size=65536 stream=live/camera utc_start=1700000000 utc_end=1700000060
```

## Short description

DVR combined audio/video export sends audio.

## Trigger condition

A recorded audio segment has a nonempty selected sample payload.

## Explanation

The payload send is attempted after this message; completion is not yet established.

## Possible causes

- A recorded audio segment has a nonempty selected sample payload.
