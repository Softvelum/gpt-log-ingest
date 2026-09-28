# HTTP live-pull settings are invalid

Product: Nimble Streamer
Component: Live input and publishing workers
Logging function: log_e
Log level: error
Documentation ID: log_e-d8a2bd5f69ffd71a

## Log pattern

```text
invalid live pull stream [url=%s, app=%s, stream=%s]
```

## Parameters

- Argument 1 — URL: Selected primary or fallback HTTP URL.
- Argument 2 — application: Configured output application, which may be empty.
- Argument 3 — stream: Configured output stream, which may be empty.

## Log examples

Illustrative examples; these are not captured server logs.

```text
invalid live pull stream [url=invalid, app=live, stream=camera]
```

## Short description

HTTP live-pull settings are invalid.

## Trigger condition

The Icecast pull URL has too few parts, an unsupported scheme, insufficient path parts for missing output names, or an invalid hostname and port.

## Explanation

The function returns without creating a receiver. It returns a true result even for these invalid settings, so the caller does not classify this as a failed connection attempt.

## Possible causes

- The selected HTTP pull URL or required output naming information is invalid.

## What to check

1. Check URL scheme, host, port, path, and output application and stream.

## Recommended action

Correct the live-pull settings.
