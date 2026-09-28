# RTMP stream name is empty

Product: Nimble Streamer
Component: RTMP command handling
Logging function: log_e
Log level: error
Documentation ID: log_e-c82307bba677aa0a

## Log pattern

```text
stream name empty
```

## Log examples

Illustrative examples; these are not captured server logs.

```text
stream name empty
```

## Short description

RTMP stream name is empty.

## Trigger condition

A play or publish request has no remaining stream name after query parameters are removed; publishing can also apply configured illegal-character replacement.

## Explanation

The stream request is rejected and connection processing fails. Query parameters alone do not provide a playable or publishable stream name.

## Possible causes

- A play or publish request has no remaining stream name after query parameters are removed; publishing can also apply configured illegal-character replacement.

## What to check

1. Check the stream name separately from its query parameters in the peer request.

## Recommended action

Supply a nonempty stream name.
