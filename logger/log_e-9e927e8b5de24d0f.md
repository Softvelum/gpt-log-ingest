# Pay-per-view API exchange failed

Product: Nimble Streamer
Component: Pay-per-view API synchronization
Logging function: log_e
Log level: error
Documentation ID: log_e-9e927e8b5de24d0f

## Log pattern

```text
PPV Api sync failed (%d)
```

## Parameters

- Argument 1 — status: Request exchange result: 0 success, 1 send failure, 2 receive failure, 3 invalid HTTP response, 4–6 TLS initialization failures, 7 partial TLS send, 8 TLS send failure, 9 TLS read failure, or 10 internal failure.

## Log examples

Illustrative examples; these are not captured server logs.

```text
PPV Api sync failed (2)
```

## Short description

Pay-per-view API exchange failed.

## Trigger condition

Sending or reading the handler request returns a nonzero exchange result.

## Explanation

Any previously applied deny list is cleared. The handler loop waits and attempts synchronization again on a later iteration.

## Possible causes

- The handler HTTP exchange failed.

## What to check

1. Interpret the status and check API reachability and response framing.

## Recommended action

Restore a valid API exchange.
