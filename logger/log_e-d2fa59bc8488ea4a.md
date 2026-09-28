# Publisher authorization exchange failed

Product: Nimble Streamer
Component: Publisher authorization
Logging function: log_e
Log level: error
Documentation ID: log_e-d2fa59bc8488ea4a

## Log pattern

```text
auth request(%s) status = %d
```

## Parameters

- Argument 1 — URL: Configured publishing authorization handler URL.
- Argument 2 — status: Request exchange result: 0 success, 1 send failure, 2 receive failure, 3 invalid HTTP response, 4–6 TLS initialization failures, 7 partial TLS send, 8 TLS send failure, 9 TLS read failure, or 10 internal failure.

## Log examples

Illustrative examples; these are not captured server logs.

```text
auth request(https://auth.example/publish) status = 2
```

## Short description

Publisher authorization exchange failed.

## Trigger condition

The authorization handler exchange returns a nonzero transport or HTTP parsing result.

## Explanation

Each pending authorization uses its configured default allow-or-deny decision instead of a handler decision.

## Possible causes

- The remote authorization exchange failed.

## What to check

1. Interpret the status and check handler connectivity and response validity.

## Recommended action

Restore the handler exchange and review the configured default decision.
