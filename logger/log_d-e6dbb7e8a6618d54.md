# DVR audio segment removal started

Product: Nimble Streamer
Component: DVR archive maintenance
Logging function: log_d
Log level: debug
Documentation ID: log_d-e6dbb7e8a6618d54

## Log pattern

```text
[DvrA::RemAS] p=%u st=%llu f='%s
```

## Parameters

- Argument 1 — Period: Unsigned recording period identifier.
- Argument 2 — Media start: 64-bit media start timestamp in the segment track timescale.
- Argument 3 — Storage file: Text naming the recorded media storage file.

## Log examples

Illustrative examples; these are not captured server logs.

```text
[DvrA::RemAS] p=1 st=90000 f='/data/dvr/live/camera/1700000000000.dat
```

## Short description

DVR audio segment removal started.

## Trigger condition

An audio-index entry is selected for deletion.

## Explanation

Its type is checked before database and file references are removed.

## Possible causes

- An audio-index entry is selected for deletion.
