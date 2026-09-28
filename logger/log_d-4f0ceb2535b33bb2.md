# Disconnection recorded for an existing stream

Product: Nimble Streamer
Component: Streaming statistics
Logging function: log_d
Log level: debug
Documentation ID: log_d-4f0ceb2535b33bb2

## Log pattern

```text
[Stat] '%s#%s' disconnected from '%s:%s:%s' (%s) cisz=%llu
```

## Parameters

- Argument 1 — Client address: Client IP address as text; an empty value can represent a request to create statistics records only.
- Argument 2 — Protocol: Protocol abbreviation used in statistics, such as HLS, RTMP or DASH.
- Argument 3 — Host: Text identifying the statistics host.
- Argument 4 — Application: Application name as text.
- Argument 5 — Stream: Stream name as text.
- Argument 6 — Recent connections: Semicolon-separated client addresses accumulated for reporting; this is not necessarily the current active-client list.
- Argument 7 — Changed streams: Number of stream statistics records awaiting reporting, supplied as a container size.

The final argument is a container size although the conversion expects unsigned long long; exact type compatibility depends on the build.

## Log examples

Illustrative examples; these are not captured server logs.

```text
[Stat] '192.0.2.10#HLS' disconnected from 'media.example:live:camera' (192.0.2.10) cisz=1
```

## Short description

Disconnection recorded for an existing stream.

## Trigger condition

A disconnection matches an existing host, application and stream record and the per-stream disconnection update has run.

## Explanation

The global client count is checked next and the stream is added to the changed-record set. The displayed changed-record count is measured before that insertion.
