# AAC frame length contains no audio payload

Product: Nimble Streamer
Component: Icecast ingest and metadata
Logging function: log_e
Log level: error
Documentation ID: log_e-c46dfdccfe211a8e

## Log pattern

```text
icecast stream aac frame size=%d invalid %s
```

## Parameters

- Argument 1 — Size: Declared ADTS frame length including header, in bytes.
- Argument 2 — URL: Configured source URL; it can be empty for an incoming publisher.

## Log examples

Illustrative examples; these are not captured server logs.

```text
icecast stream aac frame size=7 invalid http://radio.example.net/live
```

## Short description

AAC frame length contains no audio payload.

## Trigger condition

The declared ADTS frame size is at most its seven-byte header, or its nine-byte header when CRC is present.

## Explanation

The ingest session is shut down.

## Possible causes

- The declared ADTS frame size is at most its seven-byte header, or its nine-byte header when CRC is present.

## What to check

1. Check the ADTS frame length and CRC flag.
