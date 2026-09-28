# Ogg stream does not start with supported FLAC mapping

Product: Nimble Streamer
Component: Icecast ingest and metadata
Logging function: log_e
Log level: error
Documentation ID: log_e-e2bda8a691dfa997

## Log pattern

```text
unsupported ogg stream, %s
```

## Parameters

- Argument 1 — URL: Configured source URL; it can be empty for an incoming publisher.

## Log examples

Illustrative examples; these are not captured server logs.

```text
unsupported ogg stream, http://radio.example.net/live
```

## Short description

Ogg stream does not start with supported FLAC mapping.

## Trigger condition

The initial assembled Ogg payload lacks the expected Ogg FLAC identification marker.

## Explanation

Ogg processing fails and the ingest session is shut down. This parser expects FLAC within Ogg.

## Possible causes

- The initial assembled Ogg payload lacks the expected Ogg FLAC identification marker.

## What to check

1. Check the codec carried by the Ogg source.

## Recommended action

Use the supported Ogg FLAC input.
