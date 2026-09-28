# Invalid SRT password streams section

Product: Nimble Streamer
Component: SRT publishing authentication
Logging function: log_e
Log level: error
Documentation ID: log_e-a36beb75c4b156e3

## Log pattern

```text
[srt_listen_callback] failed to parse 'streams' section from password file [%s]
```

## Parameters

- Argument 1 — File: Configured SRT password-settings file path.

## Log examples

Illustrative examples; these are not captured server logs.

```text
[srt_listen_callback] failed to parse 'streams' section from password file [/data/srt-users.json]
```

## Short description

Invalid SRT password streams section.

## Trigger condition

The streams section exists but is not a JSON object.

## Explanation

Password settings processing fails and the authenticated connection is rejected.

## Possible causes

- The streams section exists but is not a JSON object.

## What to check

1. Check the streams section structure.

## Recommended action

Represent the section as the expected JSON object.
