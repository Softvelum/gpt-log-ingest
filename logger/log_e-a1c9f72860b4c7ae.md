# Invalid SRT password-settings JSON

Product: Nimble Streamer
Component: SRT publishing authentication
Logging function: log_e
Log level: error
Documentation ID: log_e-a1c9f72860b4c7ae

## Log pattern

```text
[srt_listen_callback] failed to parse password file [%s]
```

## Parameters

- Argument 1 — File: Configured SRT password-settings file path.

## Log examples

Illustrative examples; these are not captured server logs.

```text
[srt_listen_callback] failed to parse password file [/data/srt-users.json]
```

## Short description

Invalid SRT password-settings JSON.

## Trigger condition

The file content cannot be parsed as JSON or its top-level value is not an object.

## Explanation

Publisher settings cannot be applied and the authenticated connection is rejected.

## Possible causes

- The file content cannot be parsed as JSON or its top-level value is not an object.

## What to check

1. Check the password-settings file JSON syntax and root type.

## Recommended action

Provide a JSON object with the expected settings sections.
