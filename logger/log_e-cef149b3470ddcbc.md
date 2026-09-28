# HTTP media redirect limit reached

Product: Nimble Streamer
Component: Icecast ingest and metadata
Logging function: log_e
Log level: error
Documentation ID: log_e-cef149b3470ddcbc

## Log pattern

```text
max redirect count reached for url='%s' s=%d
```

## Parameters

- Argument 1 — URL: Configured source URL; it can be empty for an incoming publisher.
- Argument 2 — Socket: Numeric connection socket descriptor.

## Log examples

Illustrative examples; these are not captured server logs.

```text
max redirect count reached for url='http://radio.example.net/live' s=42
```

## Short description

HTTP media redirect limit reached.

## Trigger condition

A pulled Icecast or MPEG-TS stream returns another 301 or 302 response after five redirects have already been followed.

## Explanation

The current connection is shut down without accepting a new redirect target.

## Possible causes

- A pulled Icecast or MPEG-TS stream returns another 301 or 302 response after five redirects have already been followed.

## What to check

1. Check for a redirect loop or an unexpectedly long redirect chain.

## Recommended action

Use the final valid source URL or correct the redirect chain.
