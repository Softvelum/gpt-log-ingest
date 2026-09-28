# Origin response has no recognized playlist type

Product: Nimble Streamer
Component: HTTP client sessions
Logging function: log_e
Log level: error
Documentation ID: log_e-8e3c0286a07addd9

## Log pattern

```text
not a playlist url='%s' content-type='%s'
```

## Parameters

- Argument 1 — Misbound URL field: The format expects text, but the supplied argument is a session object pointer.
- Argument 2 — Misbound content-type field: The format labels this as content type, but receives the origin URL text.

The first %s receives a session pointer instead of text, the second receives the URL, and the content-type argument is unused. The output is undefined and cannot provide trustworthy URL or content-type fields.

## Log examples

The first %s receives a session pointer instead of text, the second receives the URL, and the content-type argument is unused. The output is undefined and cannot provide trustworthy URL or content-type fields.

## Short description

Origin response has no recognized playlist type.

## Trigger condition

A completed 200 or 206 response reaches playlist handling but its stripped URL and content type do not identify a supported playlist.

## Explanation

An HTTP 500 response is requested. The diagnostic formatting itself is invalid and can fail before that response is prepared.

## Possible causes

- A completed 200 or 206 response reaches playlist handling but its stripped URL and content type do not identify a supported playlist.

## What to check

1. Check the requested extension and origin Content-Type using another reliable request or response diagnostic.

## Recommended action

Check the requested extension and origin Content-Type using another reliable request or response diagnostic.
