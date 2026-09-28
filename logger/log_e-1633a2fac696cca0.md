# Icecast publishing content type missing

Product: Nimble Streamer
Component: Icecast ingest and metadata
Logging function: log_e
Log level: error
Documentation ID: log_e-1633a2fac696cca0

## Log pattern

```text
failed to get content type
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to get content type
```

## Short description

Icecast publishing content type missing.

## Trigger condition

A publishing request has no nonempty Content-Type header.

## Explanation

The connection is shut down before selecting the audio parser.

## Possible causes

- A publishing request has no nonempty Content-Type header.

## What to check

1. Check the publishing request headers.

## Recommended action

Provide the supported content type matching the audio payload.
