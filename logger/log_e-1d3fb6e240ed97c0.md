# SRT stream users section missing

Product: Nimble Streamer
Component: SRT publishing authentication
Logging function: log_e
Log level: error
Documentation ID: log_e-1d3fb6e240ed97c0

## Log pattern

```text
[srt_listen_callback] cannot find 'users' section in password file [%s] for [%s] stream
```

## Parameters

- Argument 1 — File: Configured SRT password-settings file path.
- Argument 2 — Stream: Requested application/stream identifier.

## Log examples

Illustrative examples; these are not captured server logs.

```text
[srt_listen_callback] cannot find 'users' section in password file [/data/srt-users.json] for [live/news] stream
```

## Short description

SRT stream users section missing.

## Trigger condition

The matching stream object has no users section.

## Explanation

Password-file authentication fails and the publisher connection is rejected.

## Possible causes

- The matching stream object has no users section.

## What to check

1. Check whether the requested stream has a users mapping.
