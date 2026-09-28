# WebRTC settings JSON cannot be parsed

Product: Nimble Streamer
Component: WebRTC settings
Logging function: log_e
Log level: error
Documentation ID: log_e-8df70e207cc9f2e1

## Log pattern

```text
failed to parse input config=[%s]
```

## Parameters

- Argument 1 — file: Path of the WebRTC settings file.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to parse input config=[/var/lib/nimble/webrtc.json]
```

## Short description

WebRTC settings JSON cannot be parsed.

## Trigger condition

A settings file loads successfully but JSON parsing returns no document.

## Explanation

No settings from this file are applied in this call; existing parameter values remain.

## Possible causes

- The loaded settings text is invalid JSON.

## What to check

1. Check the JSON syntax of the named settings file.

## Recommended action

Correct the settings document.
