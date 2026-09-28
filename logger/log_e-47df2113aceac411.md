# Cannot build Icecast chunk envelope

Product: Nimble Streamer
Component: Icecast ingest and metadata
Logging function: log_e
Log level: error
Documentation ID: log_e-47df2113aceac411

## Log pattern

```text
cannot allocate chunk buffer for [%s/%s]
```

## Parameters

- Argument 1 — Application: Application name.
- Argument 2 — Stream: Stream name.

## Log examples

Illustrative examples; these are not captured server logs.

```text
cannot allocate chunk buffer for [live/radio]
```

## Short description

Cannot build Icecast chunk envelope.

## Trigger condition

Writing the audio byte-position marker, chunk length, audio data, or attached metadata into an Icecast chunk fails.

## Explanation

In ingest setup the chunk initialization fails; in metadata forwarding the current chunk is abandoned before publication.

## Possible causes

- Writing the audio byte-position marker, chunk length, audio data, or attached metadata into an Icecast chunk fails.

## What to check

1. Check the size of the audio chunk and metadata and available buffer capacity.
