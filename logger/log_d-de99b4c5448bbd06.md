# RTMP playback resumes

Product: Nimble Streamer
Component: RTMP output and advertising
Logging function: log_d
Log level: debug
Documentation ID: log_d-de99b4c5448bbd06

## Log pattern

```text
stream resumed, client: %d
```

## Parameters

- Argument 1 — Socket: Operating-system socket descriptor for the affected connection.

## Log examples

Illustrative examples; these are not captured server logs.

```text
stream resumed, client: 42
```

## Short description

RTMP playback resumes.

## Trigger condition

Playback is no longer paused while its output sender remains in the paused state.

## Explanation

The sender returns to initial output state and enables output events to resume media.

## Possible causes

- Playback is no longer paused while its output sender remains in the paused state.

## What to check

1. Check whether a playback command intentionally resumed the session.

## Recommended action

No action is required for an expected resume.
