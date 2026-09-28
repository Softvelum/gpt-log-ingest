# Media socket shutdown started

Product: Nimble Streamer
Component: Icecast ingest and metadata
Logging function: log_d
Log level: debug
Documentation ID: log_d-6f98aac2608c8ef4

## Log pattern

```text
shutdown client s=%d
```

## Parameters

- Argument 1 — Socket: Numeric connection socket descriptor.

## Log examples

Illustrative examples; these are not captured server logs.

```text
shutdown client s=42
```

## Short description

Media socket shutdown started.

## Trigger condition

An Icecast connection or a non-TLS RTSP connection successfully requests socket shutdown.

## Explanation

The session enters its shutting-down state. The log records transport shutdown, not an application-level stream error by itself.

## Possible causes

- An Icecast connection or a non-TLS RTSP connection successfully requests socket shutdown.

## What to check

1. Use nearby diagnostics to identify why this connection was shut down.
