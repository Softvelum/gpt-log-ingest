# DVR export writes its final padding box

Product: Nimble Streamer
Component: DVR export
Logging function: log_d
Log level: debug
Documentation ID: log_d-ebc59e03d9d7adee

## Log pattern

```text
sending free atom: size=%u stream=%s utc_start=%lld utc_end=%lld
```

## Parameters

- Argument 1 — Padding box bytes: Unsigned total free-box size in bytes, including its header.
- Argument 2 — Stream: Text naming the application and stream.
- Argument 3 — UTC start: 64-bit requested UTC start in seconds since the Unix epoch; zero leaves this bound unspecified.
- Argument 4 — UTC end: 64-bit requested UTC end in seconds since the Unix epoch; zero leaves this bound unspecified.

## Log examples

Illustrative examples; these are not captured server logs.

```text
sending free atom: size=28 stream=live/camera utc_start=1700000000 utc_end=1700000060
```

## Short description

DVR export writes its final padding box.

## Trigger condition

Movie metadata has finished while the declared content length still exceeds bytes produced.

## Explanation

A free-space box fills the remaining declared MP4 length.

## Possible causes

- Movie metadata has finished while the declared content length still exceeds bytes produced.
