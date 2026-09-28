# Icecast content type unsupported

Product: Nimble Streamer
Component: Icecast ingest and metadata
Logging function: log_e
Log level: error
Documentation ID: log_e-60f5ec55ff5e86f4

## Log pattern

```text
icecast stream type not supported.url=%s, ct=%s
```

## Parameters

- Argument 1 — URL: Configured source URL; it can be empty for an incoming publisher.
- Argument 2 — Content type: Rejected content type; pull responses are lowercased before comparison.

## Log examples

Illustrative examples; these are not captured server logs.

```text
icecast stream type not supported.url=http://radio.example.net/live, ct=audio/wav
```

## Short description

Icecast content type unsupported.

## Trigger condition

The publisher request or pull response declares a content type outside the accepted audio/container set.

## Explanation

The connection is shut down. MP3, AAC, and Ogg content types are accepted; audio/mp3 is additionally accepted for pull responses.

## Possible causes

- The publisher request or pull response declares a content type outside the accepted audio/container set.

## What to check

1. Compare the declared content type with the actual encoded audio.

## Recommended action

Use a supported MP3, AAC, or Ogg input content type.
