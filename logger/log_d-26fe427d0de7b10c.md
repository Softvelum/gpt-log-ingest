# Age-based DVR cleanup selects its cutoff

Product: Nimble Streamer
Component: DVR archive maintenance
Logging function: log_d
Log level: debug
Documentation ID: log_d-26fe427d0de7b10c

## Log pattern

```text
delete outdating content of dvr stream='%s/%s' older_than=%llu
```

## Parameters

- Argument 1 — Application: Text naming the application.
- Argument 2 — Stream: Text naming the stream.
- Argument 3 — Older boundary: Unsigned UTC cutoff in seconds since the Unix epoch.

## Log examples

Illustrative examples; these are not captured server logs.

```text
delete outdating content of dvr stream='live/camera' older_than=1700000000
```

## Short description

Age-based DVR cleanup selects its cutoff.

## Trigger condition

A ready writable archive has an automatic content-age limit.

## Explanation

Content older than the cutoff is removed; a large backlog is limited to at most 24 hours of archive time per pass.

## Possible causes

- A ready writable archive has an automatic content-age limit.
