# Icecast metadata interval is too short

Product: Nimble Streamer
Component: Icecast ingest and metadata
Logging function: log_e
Log level: error
Documentation ID: log_e-e3ceedb40c64bfcb

## Log pattern

```text
icecast icy_metaint_ not supported.url=%s, icy_metaint_=%d
```

## Parameters

- Argument 1 — URL: Configured source URL; it can be empty for an incoming publisher.
- Argument 2 — Interval: Audio bytes between Icecast metadata blocks.

## Log examples

Illustrative examples; these are not captured server logs.

```text
icecast icy_metaint_ not supported.url=http://radio.example.net/live, icy_metaint_=2
```

## Short description

Icecast metadata interval is too short.

## Trigger condition

A nonzero metadata interval is less than seven bytes for AAC or less than three bytes for MP3.

## Explanation

The pull session is shut down because the audio header handling does not support that interval.

## Possible causes

- A nonzero metadata interval is less than seven bytes for AAC or less than three bytes for MP3.

## What to check

1. Check the source icy-metaint value.

## Recommended action

Use a metadata interval large enough for the selected audio format.
