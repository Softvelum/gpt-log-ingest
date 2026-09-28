# Cannot build chunk with generated Icecast metadata

Product: Nimble Streamer
Component: Icecast ingest and metadata
Logging function: log_e
Log level: error
Documentation ID: log_e-8df29ef8ab2b691c

## Log pattern

```text
cannot allocate generated_chunk buffer for [%s/%s]
```

## Parameters

- Argument 1 — Application: Application name.
- Argument 2 — Stream: Stream name.

## Log examples

Illustrative examples; these are not captured server logs.

```text
cannot allocate generated_chunk buffer for [live/radio]
```

## Short description

Cannot build chunk with generated Icecast metadata.

## Trigger condition

Writing a chunk header, audio payload, or generated metadata into the output buffer fails.

## Explanation

The current chunk is not passed to the live chunk manager.

## Possible causes

- Writing a chunk header, audio payload, or generated metadata into the output buffer fails.

## What to check

1. Check audio and generated metadata sizes and buffer allocation.
