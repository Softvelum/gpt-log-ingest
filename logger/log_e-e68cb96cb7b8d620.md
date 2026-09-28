# SRT password settings lack requested stream

Product: Nimble Streamer
Component: SRT publishing authentication
Logging function: log_e
Log level: error
Documentation ID: log_e-e68cb96cb7b8d620

## Log pattern

```text
[srt_listen_callback] cannot find [%s] stream section in password file [%s]
```

## Parameters

- Argument 1 — Stream: Requested application/stream identifier.
- Argument 2 — File: Configured SRT password-settings file path.

## Log examples

Illustrative examples; these are not captured server logs.

```text
[srt_listen_callback] cannot find [live/news] stream section in password file [/data/srt-users.json]
```

## Short description

SRT password settings lack requested stream.

## Trigger condition

The streams object has no section matching the requested application/stream.

## Explanation

The authenticated publisher connection is rejected.

## Possible causes

- The streams object has no section matching the requested application/stream.

## What to check

1. Check the stream spelling and available stream entries.
