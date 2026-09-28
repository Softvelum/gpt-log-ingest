# Session URL restriction cannot parse the request

Product: Nimble Streamer
Component: Playback session tracking
Logging function: log_e
Log level: error
Documentation ID: log_e-5ebf5f3eb6bacc17

## Log pattern

```text
failed to split URL='%s'
```

## Parameters

- Argument 1 — URL: Requested resource URL.

## Log examples

Illustrative examples; these are not captured server logs.

```text
failed to split URL='/'
```

## Short description

Session URL restriction cannot parse the request.

## Trigger condition

Session validation is enforcing URL restrictions but cannot split the current request URL.

## Explanation

Validation rejects the request before comparing its application and stream with the stored session.

## Possible causes

- The restricted request URL cannot be separated into application and stream.

## What to check

1. Compare the request URL with the original session URL.

## Recommended action

Use a correctly formed URL for the intended session.
