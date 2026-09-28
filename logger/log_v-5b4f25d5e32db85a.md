# Client disconnected after access was revoked

Product: Nimble Streamer
Component: HTTP client sessions
Logging function: log_v
Log level: verbose
Documentation ID: log_v-5b4f25d5e32db85a

## Log pattern

```text
disconnecting denied client ip='%s' ppv_id='%s' sid=%lu type=%u
```

## Parameters

- Argument 1 — Client IP: Text client address used for session tracking.
- Argument 2 — PPV client ID: Text PPV identifier, which may be empty for an API-denied session.
- Argument 3 — Session ID: Unsigned long playback-session identifier.
- Argument 4 — Protocol: Integer protocol value: 0 unknown, 1 HLS, 2 progressive download, 3 Smooth, 4 HDS, 5 MPEG-TS over HTTP, 6 Icecast, or 7 DASH.

## Log examples

Illustrative examples; these are not captured server logs.

```text
disconnecting denied client ip='192.0.2.10' ppv_id='viewer-42' sid=42 type=2
```

## Short description

Client disconnected after access was revoked.

## Trigger condition

A periodic check for progressive download, Icecast, or MPEG-TS over HTTP finds the session or PPV client denied.

## Explanation

The session is marked finished before sending more body data. Checks are spaced by at least three seconds.

## Possible causes

- A periodic check for progressive download, Icecast, or MPEG-TS over HTTP finds the session or PPV client denied.

## What to check

1. Check the corresponding API or PPV deny decision and restore access only if that denial was unintended.

## Recommended action

Check the corresponding API or PPV deny decision and restore access only if that denial was unintended.
