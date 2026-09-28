# Invalid SRT stream password section

Product: Nimble Streamer
Component: SRT publishing authentication
Logging function: log_e
Log level: error
Documentation ID: log_e-b73ed5fe20ea349e

## Log pattern

```text
[srt_listen_callback] failed to parse '%s' section from password file [%s]
```

## Parameters

- Argument 1 — Stream: Requested application/stream identifier.
- Argument 2 — File: Configured SRT password-settings file path.

## Log examples

Illustrative examples; these are not captured server logs.

```text
[srt_listen_callback] failed to parse 'live/news' section from password file [/data/srt-users.json]
```

## Short description

Invalid SRT stream password section.

## Trigger condition

The section matching the requested application/stream exists but is not a JSON object.

## Explanation

Password settings processing fails and the publisher is rejected.

## Possible causes

- The section matching the requested application/stream exists but is not a JSON object.

## What to check

1. Check the structure of the requested stream section.

## Recommended action

Represent that stream section as a JSON object.
