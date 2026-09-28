# SRT password settings lack streams section

Product: Nimble Streamer
Component: SRT publishing authentication
Logging function: log_e
Log level: error
Documentation ID: log_e-6c2c2cfc756ea91c

## Log pattern

```text
[srt_listen_callback] cannot find 'streams' section in password file [%s] for [%s] stream
```

## Parameters

- Argument 1 — File: Configured SRT password-settings file path.
- Argument 2 — Stream: Requested application/stream identifier.

## Log examples

Illustrative examples; these are not captured server logs.

```text
[srt_listen_callback] cannot find 'streams' section in password file [/data/srt-users.json] for [live/news] stream
```

## Short description

SRT password settings lack streams section.

## Trigger condition

No streams section exists in the password-settings object.

## Explanation

The authenticated publisher connection is rejected.

## Possible causes

- No streams section exists in the password-settings object.

## What to check

1. Check the password file structure.

## Recommended action

Provide the required streams mapping.
