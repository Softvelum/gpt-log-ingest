# Publisher authorization URL is invalid

Product: Nimble Streamer
Component: Publisher authorization
Logging function: log_e
Log level: error
Documentation ID: log_e-fd253b8fbb08d1f8

## Log pattern

```text
invalid url. add requests considered as failed(allow_by_default considered) url=%s
```

## Parameters

- Argument 1 — URL: Configured publishing authorization handler URL.

## Log examples

Illustrative examples; these are not captured server logs.

```text
invalid url. add requests considered as failed(allow_by_default considered) url=invalid
```

## Short description

Publisher authorization URL is invalid.

## Trigger condition

The handler URL has invalid parts, uses a scheme other than HTTP or HTTPS, or has an invalid host and port.

## Explanation

The entire request batch uses each request default allow-or-deny decision without contacting the handler.

## Possible causes

- The configured handler URL cannot be used by the HTTP request path.

## What to check

1. Check scheme, hostname, and port.

## Recommended action

Correct the publishing authorization handler URL.
