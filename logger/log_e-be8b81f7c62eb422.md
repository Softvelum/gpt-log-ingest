# RIST sender sender context creation failed

Product: Nimble Streamer
Component: RIST media integration
Logging function: log_e
Log level: error
Documentation ID: log_e-be8b81f7c62eb422

## Log pattern

```text
failed to create rist sender, id=%s
```

## Parameters

- Argument 1 — Connection ID: Configured identifier for the RIST child-process connection.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to create rist sender, id=rist-sender
```

## Short description

RIST sender sender context creation failed.

## Trigger condition

The RIST library returns a nonzero result from sender context creation.

## Explanation

No peer is added and sender initialization returns failure.

## Possible causes

- The RIST library returns a nonzero result from sender context creation.

## What to check

1. Check the RIST runtime diagnostics for sender context creation using this connection ID.

## Related log messages

- [RIST child connection entered transport initialization](log_i-66b0f0cc7975ea01.md)
