# Zixi API response received

Product: Nimble Streamer
Component: Zixi integration
Logging function: log_d
Log level: debug
Documentation ID: log_d-dfa52bacfd5d2d00

## Log pattern

```text
recv zixi response: %s
```

## Parameters

- Argument 1 — Response: Complete HTTP response body.

## Log examples

Illustrative examples; these are not captured server logs.

```text
recv zixi response: {"streams":[]}
```

## Short description

Zixi API response received.

## Trigger condition

The HTTP request callback receives a response body from the local Zixi API.

## Explanation

The complete body is logged before request-state handling and JSON validation. It does not prove the requested change succeeded.

## Possible causes

- The HTTP request callback receives a response body from the local Zixi API.

## What to check

1. Inspect the response content and any following parse or validation error.
