# Cannot buffer Icecast metadata

Product: Nimble Streamer
Component: Icecast ingest and metadata
Logging function: log_e
Log level: error
Documentation ID: log_e-efa10bc4d5d4068d

## Log pattern

```text
cannot allocate metadata buffer for [%s/%s]
```

## Parameters

- Argument 1 — Application: Application name.
- Argument 2 — Stream: Stream name.

## Log examples

Illustrative examples; these are not captured server logs.

```text
cannot allocate metadata buffer for [live/radio]
```

## Short description

Cannot buffer Icecast metadata.

## Trigger condition

Writing a metadata byte-position marker, length byte, or metadata payload fails.

## Explanation

For ingest, metadata processing fails and the session is shut down by the caller. When forwarding metadata through an encoder, construction of the current output chunk stops without publishing it.

## Possible causes

- Writing a metadata byte-position marker, length byte, or metadata payload fails.

## What to check

1. Check metadata accumulation and available buffer capacity.
