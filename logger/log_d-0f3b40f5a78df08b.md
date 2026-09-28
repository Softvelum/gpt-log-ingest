# Publisher authorization response received

Product: Nimble Streamer
Component: Publisher authorization
Logging function: log_d
Log level: debug
Documentation ID: log_d-0f3b40f5a78df08b

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
auth request(https://auth.example/publish) status = 0
```

## Short description

Publisher authorization response received.

## Trigger condition

The HTTP exchange with the authorization handler succeeds.

## Explanation

The response still must be parsed and matched to request sequence numbers before publishing is allowed or denied.
