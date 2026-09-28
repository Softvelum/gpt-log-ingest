# TLS synchronization HTTP status received

Product: Nimble Streamer
Component: Panel synchronization and proxy transport
Logging function: log_d
Log level: debug
Documentation ID: log_d-86f5fcaad81c7af0

## Log pattern

```text
sync response HTTP status code=%d
```

## Parameters

- Argument 1 — HTTP status: Parsed HTTP response status code, integer.

## Log examples

Illustrative examples; these are not captured server logs.

```text
sync response HTTP status code=200
```

## Short description

TLS synchronization HTTP status received.

## Trigger condition

A complete TLS response is parsed and silent mode is disabled.

## Explanation

TLS is shut down and released. Status 200 leads to success; other statuses lead to an invalid-response result.
