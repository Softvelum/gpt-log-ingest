# SRT publisher user absent from stream settings

Product: Nimble Streamer
Component: SRT publishing authentication
Logging function: log_e
Log level: error
Documentation ID: log_e-dfa47df0ed69fa7f

## Log pattern

```text
[srt_listen_callback] cannot find '%s' user in 'users' section in password file [%s] for [%s] stream
```

## Parameters

- Argument 1 — User: Publisher identifier from the signature or srtauth value.
- Argument 2 — File: Configured SRT password-settings file path.
- Argument 3 — Stream: Requested application/stream identifier.

## Log examples

Illustrative examples; these are not captured server logs.

```text
[srt_listen_callback] cannot find 'example' user in 'users' section in password file [/data/srt-users.json] for [live/news] stream
```

## Short description

SRT publisher user absent from stream settings.

## Trigger condition

The stream users object has no entry matching the publisher identifier.

## Explanation

The publisher is rejected without applying user-specific SRT settings.

## Possible causes

- The stream users object has no entry matching the publisher identifier.

## What to check

1. Check the authenticated user identifier and the requested stream users list.
