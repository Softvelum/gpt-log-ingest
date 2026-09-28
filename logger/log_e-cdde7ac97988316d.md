# Cannot finish Icecast Ogg chunk

Product: Nimble Streamer
Component: Icecast ingest and metadata
Logging function: log_e
Log level: error
Documentation ID: log_e-cdde7ac97988316d

## Log pattern

```text
icecast stream failed to split OGG chunk %s
```

## Parameters

- Argument 1 — URL: Configured source URL; it can be empty for an incoming publisher.

## Log examples

Illustrative examples; these are not captured server logs.

```text
icecast stream failed to split OGG chunk http://radio.example.net/live
```

## Short description

Cannot finish Icecast Ogg chunk.

## Trigger condition

After an audio frame is processed, splitting a completed chunk or preparing its successor fails.

## Explanation

The ingest session is shut down. Failure can occur while appending metadata or initializing the next chunk.

## Possible causes

- After an audio frame is processed, splitting a completed chunk or preparing its successor fails.

## What to check

1. Inspect preceding chunk and metadata buffer errors.

## Related log messages

- [Cannot build Icecast chunk envelope](log_e-47df2113aceac411.md)
- [Cannot buffer Icecast metadata](log_e-efa10bc4d5d4068d.md)
