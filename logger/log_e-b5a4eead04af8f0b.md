# RTMP authentication error has no valid transaction ID

Product: Nimble Streamer
Component: RTMP command handling
Logging function: log_e
Log level: error
Documentation ID: log_e-b5a4eead04af8f0b

## Log pattern

```text
failed to process transaction id for _error command
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to process transaction id for _error command
```

## Short description

RTMP authentication error has no valid transaction ID.

## Trigger condition

An outbound publisher awaiting a Limelight nonce or Adobe challenge receives an _error response whose AMF numeric transaction ID cannot be read.

## Explanation

The authentication response is rejected and connection processing fails before the challenge can be accepted.

## Possible causes

- An outbound publisher awaiting a Limelight nonce or Adobe challenge receives an _error response whose AMF numeric transaction ID cannot be read.

## What to check

1. Inspect the challenge response framing and transaction ID at the publish destination.

## Recommended action

Correct the malformed authentication response at the destination.
