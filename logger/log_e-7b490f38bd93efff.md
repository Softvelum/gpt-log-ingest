# Cannot initialize publisher Icecast chunk

Product: Nimble Streamer
Component: Icecast ingest and metadata
Logging function: log_e
Log level: error
Documentation ID: log_e-7b490f38bd93efff

## Log pattern

```text
icecast stream failed to init chunk.url=%s
```

## Parameters

- Argument 1 — URL: Configured source URL; it can be empty for an incoming publisher.

## Log examples

Illustrative examples; these are not captured server logs.

```text
icecast stream failed to init chunk.url=
```

## Short description

Cannot initialize publisher Icecast chunk.

## Trigger condition

After sending the successful publishing response, first-chunk initialization fails.

## Explanation

The publisher session is shut down before audio reception begins.

## Possible causes

- After sending the successful publishing response, first-chunk initialization fails.

## What to check

1. Inspect preceding metadata and chunk-buffer errors.

## Related log messages

- [Cannot build Icecast chunk envelope](log_e-47df2113aceac411.md)
- [Cannot buffer Icecast metadata](log_e-efa10bc4d5d4068d.md)
