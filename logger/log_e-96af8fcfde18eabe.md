# Invalid Icecast publishing stream URL

Product: Nimble Streamer
Component: Icecast ingest and metadata
Logging function: log_e
Log level: error
Documentation ID: log_e-96af8fcfde18eabe

## Log pattern

```text
invalid icecast stream url=%s, client=%d
```

## Parameters

- Argument 1 — URL: Publishing request URL.
- Argument 2 — Socket: Numeric connection socket descriptor.

## Log examples

Illustrative examples; these are not captured server logs.

```text
invalid icecast stream url=/radio, client=42
```

## Short description

Invalid Icecast publishing stream URL.

## Trigger condition

The publishing URL cannot be split into at least an application and a stream component.

## Explanation

The publisher connection is shut down before stream registration.

## Possible causes

- The publishing URL cannot be split into at least an application and a stream component.

## What to check

1. Check the application/stream path in the publishing URL.

## Recommended action

Publish to a valid application/stream path.
